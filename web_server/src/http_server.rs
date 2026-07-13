use crate::parser::{HttpHeaders, HttpMethod};

use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

struct HttpListener {
    path: String,
    method: HttpMethod,
    content: String,
}

pub struct HttpServer {
    server: TcpListener,
    listeners: Vec<HttpListener>,
}

impl HttpServer {
    pub fn new(port: u32) -> Self {
        let server = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();
        Self {
            server,
            listeners: Vec::new(),
        }
    }

    pub fn get(&mut self, path: &str, content: &str) {
        self.listeners.push(HttpListener {
            content: content.to_string(),
            path: path.to_string(),
            method: HttpMethod::GET,
        });
    }

    fn response(&self, stream: &mut TcpStream, content: &str, code: u32) {
        let content = format!("HTTP/1.1 {code}\r\ncontent-type text/html\r\n\r\n{content}");
        stream.write_all(content.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    async fn handle_request(&self, mut stream: TcpStream, http_headers: HttpHeaders) {
        for listener in &self.listeners {
            if listener.path == http_headers.path && listener.method == http_headers.method {
                self.response(&mut stream, &listener.content, 200);
                return;
            }
        }

        self.response(&mut stream, "not found", 404);
    }

    pub async fn listen(self) {
        let addr = self.server.local_addr().unwrap().to_string();
        println!("Sever running on address: {}", addr);
        let sv = Arc::new(self);

        for stream in sv.server.incoming() {
            let mut stream = stream.unwrap();
            let http_headers = HttpHeaders::parse(&mut stream);
            let sv2 = sv.clone();

            tokio::spawn(async move {
                sv2.handle_request(stream, http_headers).await;
            });
        }
    }
}
