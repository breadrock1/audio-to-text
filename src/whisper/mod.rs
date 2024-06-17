use actix_web::{Scope, web};

pub mod client;
pub mod client_async;
pub mod config;
pub mod forms;
pub(crate) mod resampler;
pub mod routes;
pub mod helper;

pub fn build_scope() -> Scope {
    web::scope("/recognize")
        .service(routes::upload_form)
        .service(routes::recognize_file)

}
