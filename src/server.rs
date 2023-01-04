use crate::http::{Request, Response, StatusCode, ParseError};
// include traits to use them.
use std::convert::TryFrom;
// use std::convert::TryInto;
use std::io::Read;
use std::net::TcpListener;
// Crate is called from root module, main

pub struct Server {
    addr: String,
}

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(StatusCode::Ok, Some("<h1>Test</h1>".to_string()))
    }
    fn handle_bad_request(&mut self, error: &ParseError) -> Response {
        println!("Failed to parse request: {}", error);
        Response::new(StatusCode::BadRequest, None)
    }
}

// Implementation for Server
impl Server {

    // Create new instance of server
    pub fn new(addr: String) -> Self {
        Self {
            addr,
        }
    }

    pub fn run(self,  mut handler: impl Handler) {
        println!("Listening on {} 👂🏼", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // 1024 one kilobyte is enough for a test.
                    let mut buffer = [0; 1024];
                    // convert buffer byte array in to a request.
                    let response = match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            // Result wrapping a request
                            // res needs a explicit return type
                            // let res: &Result<Request, _> = &buffer[..].try_into();

                            // Explicit conversion to byte slice
                            // Can directly create a slice that contains the entire array
                            // Emit lower and upper bounds with [..] to create a bite slice that
                            // Contains the entire array.
                            // Same as Request::try_from(&buffer as &[u8])
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response");
                            }
                        }
                        Err(e) => println!("Failed to read from connection {}", e)
                    };
                }
                Err(err) => println!("Failed to establish a connection: {}", err)
            }

            let res = listener.accept();

            if res.is_err() {
                continue;
            }

            let (stream, addr) = res.unwrap();
        }
    }
}



