use std::net::{IpAddr, SocketAddr};

use config::File;
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub struct Config {
    pub socket_addrs: Vec<SocketAddr>,
}

impl Config {
    pub fn new() -> Self {
        let mut config = config::Config::default();
        config.merge(File::with_name("config/config")).unwrap();

        config.try_into().unwrap()
    }
}

impl<'de> Deserialize<'de> for Config {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            SocketAddrs,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("'socket-addresses'")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "socket-addresses" => Ok(Field::SocketAddrs),
                            _ => Err(de::Error::unknown_field(value, &["socket-addresses"])),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ConfigVisitor;

        impl<'de> Visitor<'de> for ConfigVisitor {
            type Value = Config;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Config")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut socket_addrs: Option<Vec<SocketAddrWrapper>> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::SocketAddrs => {
                            socket_addrs = Some(map.next_value()?);
                            break;
                        }
                    }
                }

                let socket_addrs =
                    socket_addrs.ok_or_else(|| de::Error::missing_field("socket-addresses"))?;
                let socket_addrs = socket_addrs.iter().map(|wrapper| wrapper.0).collect();
                Ok(Config { socket_addrs })
            }
        }

        const FIELDS: &'static [&'static str] = &["socket_addrs"];
        deserializer.deserialize_struct("Config", FIELDS, ConfigVisitor)
    }
}

struct SocketAddrWrapper(SocketAddr);

impl<'de> Deserialize<'de> for SocketAddrWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Host,
            Port,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("'host' or 'port'")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "host" => Ok(Field::Host),
                            "port" => Ok(Field::Port),
                            _ => Err(de::Error::unknown_field(value, &["host", "port"])),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct SocketAddrWrapperVisitor;

        impl<'de> Visitor<'de> for SocketAddrWrapperVisitor {
            type Value = SocketAddrWrapper;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an array of socket addresses")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut host: Option<String> = None;
                let mut port: Option<u16> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Host => {
                            if host.is_some() {
                                return Err(de::Error::duplicate_field("host"));
                            }
                            host = Some(map.next_value()?);
                        }
                        Field::Port => {
                            if port.is_some() {
                                return Err(de::Error::duplicate_field("port"));
                            }
                            port = Some(map.next_value()?);
                        }
                    }
                }

                let host = host.ok_or_else(|| de::Error::missing_field("host"))?;
                // Can't use config::Config::set_default because implementation is not sound
                // TODO: extract magic number
                let port = port.unwrap_or(80);

                Ok(SocketAddrWrapper(SocketAddr::new(
                    host.parse::<IpAddr>().unwrap(),
                    port,
                )))
            }
        }

        deserializer.deserialize_tuple_struct("SocketAddrWrapper", 1, SocketAddrWrapperVisitor)
    }
}
