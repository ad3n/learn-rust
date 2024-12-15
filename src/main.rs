mod app;

use app::config;
use app::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = config::read_config(String::from("config.json"));

    server::create_server(config).await
}
