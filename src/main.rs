#[macro_use]
extern crate diesel;

use crate::swagger_docs::ApiDoc;
use actix_web::App;
use actix_web::HttpServer;
use controller::chat::azure_controller;
use controller::conversation::conversation_controller;
use controller::conversation::conversation_item_controller;
use controller::monitor::health_controller;
use rust_wheel::config::app::app_conf_reader::get_app_config;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;

pub mod common;
pub mod controller;
pub mod model;
pub mod service;
mod swagger_docs;
mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let port: u16 = get_app_config("chat.port").parse().unwrap();
    let address = ("0.0.0.0", port);
    HttpServer::new(|| {
        App::new()
            .configure(azure_controller::config)
            .configure(health_controller::config)
            .configure(conversation_controller::config)
            .configure(conversation_item_controller::config)
            .service(
                SwaggerUi::new("/docs-v1/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
    })
    .workers(3)
    .bind(address)?
    .run()
    .await
}
