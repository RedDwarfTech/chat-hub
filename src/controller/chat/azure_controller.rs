use actix_web::{post, web, Responder};
use rust_wheel::common::wrapper::actix_http_resp::box_actix_rest_response;

/// User login
///
/// user login
#[utoipa::path(
    context_path = "/infra/user/login",
    path = "/",
    responses(
        (status = 200, description = "support user login")
    )
)]
#[post("/login")]
pub async fn login() -> impl Responder {
    return box_actix_rest_response("ok");
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/ai/azure").service(login);
    conf.service(scope);
}
