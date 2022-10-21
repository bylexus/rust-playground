mod httpserver;
use httpserver::HttpServer;

fn main() {
    let server = HttpServer::new("127.0.0.1:3000");
    server.start().unwrap();
}
