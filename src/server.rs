use std::net::SocketAddr;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

use crate::app::services;

pub fn run(socket_addrs: Vec<SocketAddr>) -> std::io::Result<actix_web::dev::Server> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(services::control)
            .service(services::shutdown)
    })
    .bind(&*socket_addrs)
    .map(|server| server.run())
}
