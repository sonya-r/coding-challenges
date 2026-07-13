use std::io::{BufRead, BufReader};
use std::net::TcpStream;

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
}
pub struct HttpHeaders {
    // version: f32,
    pub method: HttpMethod,
    pub path: String,
}

impl HttpHeaders {
    pub fn parse(stream: &mut TcpStream) -> Self {
        let mut reader = BufReader::new(stream);
        let data = reader.fill_buf().unwrap().to_vec();
        reader.consume(data.len());

        let data = String::from_utf8(data).unwrap();
        let mut lines: Vec<_> = data.lines().collect();

        if lines.len() == 0 {
            panic!("todo");
        }

        let http_info: Vec<_> = lines.remove(0).split_whitespace().collect();

        if http_info.len() != 3 {
            eprint!("{http_info:?}");
            panic!("todo");
        }

        let method;
        let path = http_info[1].to_string();
        // let version = http_info[2];

        if http_info[0].to_lowercase().trim() == "get" {
            method = HttpMethod::GET;
        } else {
            panic!("todo")
        }

        Self { method, path: path }

        // for line in lines {
        //     let res = line.split(":");
        // }
    }
}
