// include server module and http module.
mod server;
mod http;
mod website_handler;

// include Server struct from server
use server::Server;
use website_handler::WebsiteHandler;
fn main() {
    // Call and create a new instance
    let server = Server::new("127.0.0.1:8080".to_string());
    // Call and run on Server.
    server.run(WebsiteHandler);
}
