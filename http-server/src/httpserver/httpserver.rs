use http_server::utils::threadpool::ThreadPool;

use crate::httpserver::Request;

use std::{
    error::Error,
    io::Write,
    net::{TcpListener, TcpStream},
};

pub struct HttpServer {
    bind_addr: String,
    thread_pool: ThreadPool,
    // tcp_listener: Option<TcpListener>,
}

impl HttpServer {
    pub fn new(bind_addr: &str) -> HttpServer {
        let tpool = ThreadPool::builder(5);
        eprintln!("Started 5 request handling threads");
        HttpServer {
            bind_addr: String::from(bind_addr),
            thread_pool: tpool, // tcp_listener: None,
        }
    }

    pub fn start(&self) -> Result<(), Box<dyn Error>> {
        let tcp_listener = TcpListener::bind(&self.bind_addr)?;

        eprintln!("Server started on {}", self.bind_addr);
        for stream in tcp_listener.incoming() {
            let stream = match stream {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Cannot read incoming stream: {}", e);
                    continue;
                }
            };
            self.handle_incoming_stream(stream);
        }
        Ok(())
    }

    fn handle_incoming_stream(&self, mut stream: TcpStream) {
        self.thread_pool.execute(move |thread_id| {
            let request = Request::from_tcp_stream(&stream);

            request.params.pairs().for_each(|(key, value)| {
                eprintln!("URL Param: {}: {}", key, value);
            });

            // TODO: read further request in the SAME stream: maybe this is a
            // keep-alive-connection.

            // stream.shutdown(std::net::Shutdown::Read).unwrap();

            eprintln!("Thread {} handles the Request", thread_id);

            let mut response = String::from("HTTP/1.1 200 OK \r\n\r\n");
            response += format!("{:?} request read.\n", request.method).as_str();
            if let Some(body) = request.body {
                response += format!("Body:\n{}\n", body).as_str();
            }
            stream.write_all(response.as_bytes()).unwrap();
        });
    }
}
