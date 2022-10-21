use std::str;
use std::{
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

use crate::httpserver::HeaderMap;

use super::RequestParams;

#[derive(Debug)]
pub enum HttpVerb {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
    UNKNOWN,
}

impl HttpVerb {
    fn from(string: &str) -> HttpVerb {
        match string.to_lowercase().as_str() {
            "get" => HttpVerb::GET,
            "head" => HttpVerb::HEAD,
            "post" => HttpVerb::POST,
            "put" => HttpVerb::PUT,
            "delete" => HttpVerb::DELETE,
            "connect" => HttpVerb::CONNECT,
            "options" => HttpVerb::OPTIONS,
            "trace" => HttpVerb::TRACE,
            "patch" => HttpVerb::PATCH,
            _ => HttpVerb::UNKNOWN,
        }
    }
}

pub struct Request {
    pub headers: HeaderMap,
    pub method: HttpVerb,
    pub url: String,
    pub body: Option<String>,
	pub params: RequestParams,
}

impl Request {
    pub fn from_tcp_stream(stream: &TcpStream) -> Request {
        let mut buf_reader = BufReader::new(stream);
        let mut http_request_line = String::new();
        let mut header_buf = String::new();
        let mut headers = Vec::new();
        let mut verb = HttpVerb::UNKNOWN;
        let mut url = String::new();

        // read 1st line: http request and verb:
        if let Ok(_) = buf_reader.read_line(&mut http_request_line) {
            let line = http_request_line.trim();
            (verb, url) = Request::parse_http_request_line(&line);
        }

        // Read header lines:
        while let Ok(_) = buf_reader.read_line(&mut header_buf) {
            let line = header_buf.trim();
            if line.is_empty() {
                // header end reached
                break;
            } else {
                headers.push(String::from(line));
                header_buf.clear();
            }
        }
        let header_map = HeaderMap::builder(&headers);
        let mut request = Request {
            headers: header_map,
            method: verb,
            url:String::from(&url),
            body: None,
			params: RequestParams::from_request_url(&url)
        };
        // eprintln!("Headers: {:#?}\n", request.headers);

        // now read the request body, if any is given:
        if let Some(bytes_str) = request.headers.get("content-length") {
            let bytes: usize = match bytes_str.parse() {
                Ok(val) => val,
                Err(_) => 0,
            };

            if bytes > 0 {
                let mut buf = vec![0u8; bytes];
                let mut body = String::new();
                if let Ok(_) = buf_reader.read_exact(&mut buf) {
                    body += match std::str::from_utf8(&buf) {
                        Ok(res) => res,
                        Err(_) => "",
                    };
                    request.body = Some(body);

                    // std::str::from_utf8(buf);
                }
            }

            // TODO: read
        }

        request
    }

    fn parse_http_request_line(line: &str) -> (HttpVerb, String) {
        let mut verb = HttpVerb::UNKNOWN;
        let mut url = String::new();
        // TODO: HTTP Version extraction

        let parts: Vec<_> = line.split_ascii_whitespace().collect();
        if parts.len() > 0 {
            verb = HttpVerb::from(parts[0]);
        }
        if parts.len() > 1 {
            url = String::from(parts[1]);
        }

        (verb, url)
    }
}
