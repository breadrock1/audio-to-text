use crate::healthcheck::routes::check_health;

use actix_web::{Scope, web};

pub mod routes;

pub fn build_scope() -> Scope {
    web::scope("/healthcheck").service(check_health)
}
