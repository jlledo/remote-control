#![windows_subsystem = "windows"]

use remote_control::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new();
    remote_control::run(config)?.await
}
