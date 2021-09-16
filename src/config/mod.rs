use std::net::SocketAddr;

use crate::config::raw::{RawConfig, RawSocketAddr};

mod raw;

#[derive(Debug)]
pub struct Config {
    pub control_server: ServerConfig,
    pub shutdown_server: ServerConfig,
}

impl Config {
    pub fn new() -> Self {
        let raw_config = RawConfig::new();

        Config {
            control_server: raw_config.server.control.into(),
            shutdown_server: raw_config.server.shutdown.into(),
        }
    }
}

#[derive(Debug)]
pub struct ServerConfig {
    pub socket_addrs: Vec<SocketAddr>,
}

impl From<Vec<RawSocketAddr>> for ServerConfig {
    fn from(addrs: Vec<RawSocketAddr>) -> Self {
        ServerConfig {
            socket_addrs: addrs.into_iter().map(|addr| addr.into()).collect(),
        }
    }
}
