use crate::httpserver::{HTTPStatusCode, Request};
use http_server::utils::logging::LogSeverity;
use http_server::utils::threadpool::ThreadPool;

use std::error::Error as StdError;
use std::fmt::{Error, Formatter};
use std::result::Result as StdResult;
use std::{
    fmt::Display,
    io::Write,
    net::{TcpListener, TcpStream},
};

pub struct HttpServer {
    bind_addr: String,
    thread_pool: ThreadPool,
}

impl HttpServer {
    pub fn new(bind_addr: &str) -> HttpServer {
        let tpool = ThreadPool::builder(5);
        eprintln!("Started 5 request handling threads");
        HttpServer {
            bind_addr: String::from(bind_addr),
            thread_pool: tpool,
        }
    }

    pub fn start(&self) -> StdResult<(), Box<dyn StdError>> {
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

    fn handle_incoming_stream(&self, stream: TcpStream) {
        self.thread_pool.execute(move |thread_id| {
            eprintln!("Thread {} handles the Request", thread_id);

            // let handler = RequestHander::from_tcp_stream(&stream);

            let request = Request::from_tcp_stream(stream);
            match request {
                Ok(mut request) => request.handle(),
                Err(e) => {
                    Self::log(e.message(), LogSeverity::ERROR)
                }
            }

            // from_tcp_stream takes ownership of the stream, while handle() gives it
            // back after its work is done:
            // let buf_reader = handler.handle();

            // TODO: read further request in the SAME stream: maybe this is a
            // keep-alive-connection.
            // we therefore get the buf_reader back from the handler.
            // for now, we just stop here.

            // stream.shutdown(std::net::Shutdown::Read).unwrap();
        });
    }

    fn log(msg: &str, severity: LogSeverity) {
        eprintln!("{}: {}\n", severity, msg);
    }
}
