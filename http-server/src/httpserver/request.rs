use std::error::Error;
use std::fmt::Display;
use std::io::ErrorKind;
use std::str;
use std::{
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

use crate::httpserver::{HeaderMap, RequestParams};
use http_server::utils::BufReaderExt;

use super::HTTPStatusCode;

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
    pub full_url: String,
    pub url: String,
    pub body: Option<String>,
    pub params: RequestParams,
}

impl Request {
    /// Creates a Request from the given stream. As this stream is possibly from a keep-alive
    /// connection, we also return the still opened Buffered Reader, so that another request
    /// can be established from the already read-in-progress buffer.
    pub fn from_tcp_stream<'a>(
        stream: &'a TcpStream,
    ) -> Result<(Request, BufReader<&'a TcpStream>), HTTPStatusCode> {
        let mut buf_reader = BufReader::new(stream);
        let mut header_buf = String::new();
        let mut headers = Vec::new();
        let mut verb = HttpVerb::UNKNOWN;
        let mut url = String::new();

        // read 1st line: http request and verb:
        let line_buf = match buf_reader.read_max_until(10, 8192) {
            Ok(buf) => buf,
            Err(err) => match err.kind() {
                ErrorKind::UnexpectedEof => return Err(HTTPStatusCode::ClientError(413)),
                _ => return Err(HTTPStatusCode::ServerError(500))
            }
        };
        let line = String::from_utf8(line_buf).unwrap_or(String::new());
        (verb, url) = Request::parse_http_request_line(line.trim());

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
            full_url: String::from(&url),
            // url only contains the url part without parameters:
            url: String::from(match url.split_once('?') {
                Some(parts) => parts.0,
                None => &url,
            }),
            body: None,
            params: RequestParams::from_request_url(&url),
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

        Ok((request, buf_reader))
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
