use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use awc::Client;
use std::collections::HashMap;

async fn handler(
    request: HttpRequest,
    query: web::Query<HashMap<String, String>>,
    payload: web::Payload,
) -> impl Responder {
    let mut client = Client::default().post("https://httpbin.org/anything");

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
       App::new()
        .route("/static/{id}", web::get().to(handler))
        .route("/static/{id}", web::post().to(handler))
        .route("/static/{id}", web::put().to(handler))
        .route("/static/{id}", web::patch().to(handler))
        .route("/static/{id}", web::delete().to(handler))
    })
    .workers(17)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
