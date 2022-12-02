// include server module and http module.
mod server;
mod http;

// include Server struct from server
use server::Server;

fn main() {
    // Call and create a new instance
    let server = Server::new("127.0.0.1:8080".to_string());
    // Call and run on Server.
    server.run();
}