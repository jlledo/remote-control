use actix_web::dev::Server;

use crate::config::Config;

pub mod config;
mod control;
pub mod log;
mod shutdown;

pub fn run(config: Config) -> std::io::Result<Server> {
    control::run_server(config.control_server.socket_addrs);
    shutdown::run_server(config.shutdown_server.socket_addrs)
}
