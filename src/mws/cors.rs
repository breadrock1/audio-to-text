use actix_cors::Cors;
use actix_web::http::header;

pub fn build_cors_policy() -> Cors {
    let available_methods = vec!["GET", "POST", "OPTIONS"];
    let available_headers = vec![header::AUTHORIZATION, header::ACCEPT];

    Cors::default()
        .allowed_header(header::CONTENT_TYPE)
        .allowed_methods(available_methods)
        .allowed_headers(available_headers)
        .allow_any_origin()
        .max_age(3600)
}
