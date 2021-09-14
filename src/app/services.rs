use actix_web::web::Payload;
use actix_web::{get, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use log::info;
use system_shutdown;

use crate::app::socket::ControlSocket;

#[get("/shutdown")]
pub async fn shutdown() -> impl Responder {
    match system_shutdown::shutdown() {
        Ok(_) => HttpResponse::Ok().body("Shutting down, bye!"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to shut down: {}", e)),
    }
}

#[get("/control")]
pub async fn control(r: HttpRequest, stream: Payload) -> actix_web::Result<HttpResponse> {
    let res = ws::start(ControlSocket::new(), &r, stream);
    info!("Established control websocket connection");
    res
}
