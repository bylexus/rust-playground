use http_server::utils::logging::LogSeverity;
use std::{
    io::{BufReader, Read, Write},
    net::TcpStream,
};

use crate::httpserver::{HTTPStatusCode, Request};

trait Stream: Write + Read {}

pub struct RequestHander<'a> {
    stream: &'a TcpStream,
}

impl<'a> RequestHander<'a> {
    pub fn from_tcp_stream(stream: &'a TcpStream) -> RequestHander {
        RequestHander { stream }
    }

    pub fn handle(mut self) -> Option<BufReader<&'a TcpStream>> {
        match Request::from_tcp_stream(&mut self.stream) {
            Ok((request, buffered_reader)) => {
                let code = HTTPStatusCode::Success(200);
                self.write_response_code(code);

                // output header
                // TODO: More to come...
                self.write_header("Connection", "close");

                let mut response = format!("{:?} request read.\n", request.method);

                if let Some(body) = request.body {
                    response += format!("Body:\n{}", body).as_str();
                }

                // End header:
                self.write_header("Content-Length", &format!("{}", response.len()));
                self.write_to_response_stream("\r\n");

                // output body
                self.write_to_response_stream(&response);

                // return
                Some(buffered_reader)
            }
            Err(http_err) => {
                let response = String::from(format!(
                    "HTTP/1.1 {} {} \r\n",
                    http_err.code(),
                    http_err.message()
                ));
                self.write_response_code(http_err);

                // return
                None
            }
        }
    }

    fn write_response_code(&mut self, code: HTTPStatusCode) {
        self.write_to_response_stream(&format!("HTTP/1.1 {} {} \r\n", code.code(), code.message()));
    }

    fn write_header(&mut self, key: &str, value: &str) {
        let header_str = format!("{}: {}\r\n", key, value);
        self.write_to_response_stream(&header_str);
    }

    fn write_to_response_stream(&mut self, data: &str) {
        match self.stream.write(data.as_bytes()) {
            Ok(_) => return,
            Err(e) => self.log(&e.to_string(), LogSeverity::ERROR),
        };
    }

    fn log(&self, msg: &str, severity: LogSeverity) {
        eprintln!("{}: {}\n", severity, msg);
    }
}
