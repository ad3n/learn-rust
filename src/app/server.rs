use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};

use super::route;
use super::config;

pub fn create_server(config: config::Config) -> actix_web::dev::Server {
    let config_mutex = Arc::new(Mutex::new(config.endpoints));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config_mutex.clone()))
            .configure(|cfg| route::configure_routes(cfg, web::Data::new(config_mutex.clone()))) // Use the route configuration
    })
    .bind(config.address)
    .expect("Cannot bind to port")
    .run()
}
