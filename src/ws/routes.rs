use crate::ws::actor::WebsocketActor;
use crate::whisper::config::WhisperClientConfig;

use actix_web::{Error, HttpRequest, HttpResponse, Responder, web};
use actix_web::http::StatusCode;
use actix_web_actors::ws;

pub async fn stream(_: HttpRequest) -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/stream.html"))
}

pub async fn websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let wh_cfg = WhisperClientConfig::default();
    ws::start(WebsocketActor::new(&wh_cfg), &req, stream)
}
