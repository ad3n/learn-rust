use actix_web::{web, HttpRequest};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use super::config;
use super::handler;

pub fn configure_routes(cfg: &mut web::ServiceConfig, endpoints: web::Data<Arc<Mutex<HashMap<String, config::Endpoint>>>>) {
    for (route, endpoint) in  endpoints.lock().unwrap().clone().iter() {
        let endpoint = endpoint.clone();

        match endpoint.method.to_lowercase().as_str() {
            "get" => {
                cfg.route(route, web::get().to(move |req: HttpRequest, query: web::Query<HashMap<String, String>>, payload: web::Payload| {
                    handler::handle(endpoint.clone(), req, query, payload)
                }));
            },
            "post" => {
                cfg.route(route, web::post().to(move |req: HttpRequest, query: web::Query<HashMap<String, String>>, payload: web::Payload| {
                    handler::handle(endpoint.clone(), req, query, payload)
                }));
            },
            "put" => {
                cfg.route(route, web::put().to(move |req: HttpRequest, query: web::Query<HashMap<String, String>>, payload: web::Payload| {
                    handler::handle(endpoint.clone(), req, query, payload)
                }));
            },
            "patch" => {
                cfg.route(route, web::patch().to(move |req: HttpRequest, query: web::Query<HashMap<String, String>>, payload: web::Payload| {
                    handler::handle(endpoint.clone(), req, query, payload)
                }));
            },
            "delete" => {
                cfg.route(route, web::delete().to(move |req: HttpRequest, query: web::Query<HashMap<String, String>>, payload: web::Payload| {
                    handler::handle(endpoint.clone(), req, query, payload)
                }));
            },
            _ => {
                cfg.route(route, web::to(move |req: HttpRequest, query: web::Query<HashMap<String, String>>, payload: web::Payload| {
                    handler::not_allowed(endpoint.clone(), req, query, payload)
                }));
            }
        }
    }
}

