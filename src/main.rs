#![windows_subsystem = "windows"]

use remote_control::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    remote_control::log::init();
    let config = Config::default();

    remote_control::run(config)?.await
}
