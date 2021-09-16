use crate::config::Config;

pub mod config;
mod control;
mod log;
mod shutdown;

pub fn run(config: Config) -> std::io::Result<actix_web::dev::Server> {
    log::init();
    control::run_server(&config.socket_addrs)?;
    shutdown::run_server(&config.socket_addrs)
}
