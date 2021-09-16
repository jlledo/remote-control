use std::net::{IpAddr, SocketAddr};

use config::File;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(super) struct RawConfig {
    pub(super) server: RawServerConfigs,
}

impl RawConfig {
    pub(super) fn new() -> Self {
        let mut config = config::Config::default();
        config.merge(File::with_name("config/config")).unwrap();

        config.try_into().unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub(super) struct RawServerConfigs {
    pub(super) control: Vec<RawSocketAddr>,
    pub(super) shutdown: Vec<RawSocketAddr>,
}

#[derive(Deserialize, Debug)]
pub(super) struct RawSocketAddr {
    host: String,
    port: u16,
}

impl From<RawSocketAddr> for SocketAddr {
    fn from(addr: RawSocketAddr) -> Self {
        SocketAddr::new(addr.host.parse::<IpAddr>().unwrap(), addr.port)
    }
}
