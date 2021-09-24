use std::net::SocketAddr;

use actix_web::middleware::Logger;
use actix_web::{get, HttpResponse, Responder};
use actix_web::{App, HttpServer};

#[get("/shutdown")]
pub async fn shutdown() -> impl Responder {
    match system_shutdown::shutdown() {
        Ok(_) => HttpResponse::Ok().body("Shutting down, bye!"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to shut down: {}", e)),
    }
}

pub fn run_server(socket_addrs: Vec<SocketAddr>) -> std::io::Result<actix_web::dev::Server> {
    HttpServer::new(|| App::new().wrap(Logger::default()).service(shutdown))
        .bind(&*socket_addrs)
        .map(|server| server.run())
}
