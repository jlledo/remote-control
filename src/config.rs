use std::net::{IpAddr, SocketAddr};

use config::File;
use serde::Deserialize;

#[derive(Debug)]
pub struct Config {
    pub socket_addrs: Vec<SocketAddr>,
}

impl Config {
    const DEFAULT_PORT: u16 = 80;

    pub fn new() -> Self {
        let raw_config = RawConfig::new();
        let socket_addrs = raw_config
            .socket_addrs
            .iter()
            .map(|raw_addr| {
                SocketAddr::new(
                    raw_addr.host.parse::<IpAddr>().unwrap(),
                    raw_addr.port.unwrap_or(Self::DEFAULT_PORT),
                )
            })
            .collect();

        Config { socket_addrs }
    }
}
#[derive(Deserialize, Debug)]
struct RawConfig {
    #[serde(rename = "socket-addresses")]
    socket_addrs: Vec<RawSocketAddr>,
}
#[derive(Deserialize, Debug)]
struct RawSocketAddr {
    host: String,
    port: Option<u16>,
}

impl RawConfig {
    fn new() -> Self {
        let mut config = config::Config::default();
        config.merge(File::with_name("config/config")).unwrap();

        config.try_into().unwrap()
    }
}
