use httpd::server::Server;
use httpd::http::{Request, Method};



fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run()
}

// HTTP request
// GET /user?id=10 HTTP/1.1\r\n
// HEADERS \r\n
// BODY