use std::io::ErrorKind;
use std::str;
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
};

use crate::httpserver::{HeaderMap, RequestParams};
use http_server::utils::logging::LogSeverity;
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
    tcp_stream: TcpStream,
    buf_reader: BufReader<TcpStream>,
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
    pub fn from_tcp_stream(stream: TcpStream) -> Result<Request, HTTPStatusCode> {
        let stream_copy = match stream.try_clone() {
            Ok(s) => s,
            Err(_) => return Err(HTTPStatusCode::ServerError(500)),
        };
        let mut buf_reader = BufReader::new(stream_copy);
        let mut header_buf = String::new();
        let mut headers = Vec::new();

        // read 1st line: http request and verb:
        let line_buf = match buf_reader.read_max_until(10, 8192) {
            Ok(buf) => buf,
            Err(err) => match err.kind() {
                ErrorKind::UnexpectedEof => return Err(HTTPStatusCode::ClientError(413)),
                _ => return Err(HTTPStatusCode::ServerError(500)),
            },
        };
        let line = String::from_utf8(line_buf).unwrap_or(String::new());
        let (verb, url) = Request::parse_http_request_line(line.trim());

        let full_url = String::from(&url);
        let url = String::from(match url.split_once('?') {
            Some(parts) => parts.0,
            None => &url,
        });
        let params = RequestParams::from_request_url(&url);

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
            tcp_stream: stream,
            buf_reader: buf_reader,
            headers: header_map,
            method: verb,
            full_url: full_url,
            url: url,
            body: None,
            params: params,
        };

        // now read the request body, if any is given:
        if let Some(bytes_str) = request.headers.get("content-length") {
            let bytes: usize = match bytes_str.parse() {
                Ok(val) => val,
                Err(_) => 0,
            };

            if bytes > 0 {
                let mut buf = vec![0u8; bytes];
                let mut body = String::new();
                if let Ok(_) = request.buf_reader.read_exact(&mut buf) {
                    body += match std::str::from_utf8(&buf) {
                        Ok(res) => res,
                        Err(_) => "",
                    };
                    request.body = Some(body);
                }
            }
        }

        Ok(request)
    }

    pub fn handle(&mut self) {
        let code = HTTPStatusCode::Success(200);
        self.write_response_code(code);

        // output header
        // TODO: More to come...
        // TODO: Connection header should not be used, see https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Connection
        self.write_header("Connection", "close");

        let mut response = format!("{:?} request read.\n", self.method);

        if let Some(body) = &self.body {
            response += format!("Body:\n{}", body).as_str();
        }

        // End header:
        self.write_header("Content-Length", &format!("{}", response.len()));
        self.write_to_response_stream("\r\n");

        // output body
        self.write_to_response_stream(&response);

        // TODO: read further request in the SAME stream: maybe this is a
        // keep-alive-connection.
        // we therefore get the buf_reader back from the handler.
        // for now, we just stop here.

        self.tcp_stream.shutdown(std::net::Shutdown::Read).unwrap();

        // return
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

    fn write_response_code(&mut self, code: HTTPStatusCode) {
        self.write_to_response_stream(&format!("HTTP/1.1 {} {} \r\n", code.code(), code.message()));
    }

    fn write_header(&mut self, key: &str, value: &str) {
        let header_str = format!("{}: {}\r\n", key, value);
        self.write_to_response_stream(&header_str);
    }

    fn write_to_response_stream(&mut self, data: &str) {
        match self.tcp_stream.write(data.as_bytes()) {
            Ok(_) => return,
            Err(e) => self.log(&e.to_string(), LogSeverity::ERROR),
        };
    }

    fn log(&self, msg: &str, severity: LogSeverity) {
        eprintln!("{}: {}\n", severity, msg);
    }
}
