use std::fs;
use web_server::http_server::HttpServer;

#[tokio::main]
async fn main() {
    let mut server = HttpServer::new(8080);
    let content = fs::read_to_string("index.html").unwrap();

    server.get("/", &content);

    server.listen().await;
}
