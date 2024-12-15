use actix_web::{web, HttpRequest, HttpResponse, Responder};
use awc::Client;
use std::collections::HashMap;

use super::config;

pub async fn handle(
    endpoint: config::Endpoint,
    request: HttpRequest,
    query: web::Query<HashMap<String, String>>,
    payload: web::Payload,
) -> impl Responder {
    let mut client = Client::default().post(endpoint.host + endpoint.path.as_str());

    for (key, value) in request.headers() {
        client = client.insert_header((key, value));
    }

    let mut query_params = query.into_inner();
    let request_uri: Vec<&str> = request.path().split("/").collect();
    let mut params: HashMap<String, String> = HashMap::new();
    for (k, v) in request.match_pattern().unwrap().split("/").enumerate() {
        let m = String::from(v);
        let p = String::from(request_uri[k]);
        if m == p {
            continue;
        }

        params.insert(m, p);
    }
    
    query_params.insert("path".to_string(), request.path().to_string());
    query_params.insert("match".to_string(), request.match_pattern().unwrap());

    let mut res = client 
    .insert_header(("X-ServerID", "Aden Kejawen Server"))
    .query(&query_params)
    .expect("failed to add query params")
    .send_body(payload.to_bytes().await.unwrap())                            
    .await
    .unwrap();

    let body= res.body().await.unwrap();
    let content_type = res.headers().get("Content-Type");

    HttpResponse::Ok().content_type(content_type.unwrap()).body(body)
}

pub async fn not_allowed(
    _: config::Endpoint,
    _: HttpRequest,
    _: web::Query<HashMap<String, String>>,
    _: web::Payload,
) -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body("method not allowed")
}
