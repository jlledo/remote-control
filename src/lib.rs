use crate::config::Config;

mod app;
pub mod config;
mod log;
mod server;

pub fn run(config: Config) -> std::io::Result<actix_web::dev::Server> {
    log::init();
    server::run(config.socket_addrs)
}
