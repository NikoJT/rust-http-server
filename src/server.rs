// include traits to use them.
use std::convert::TryFrom;
// use std::convert::TryInto;
use std::io::Read;
use std::net::TcpListener;
// Crate is called from root module, main
use crate::http::Request;

pub struct Server {
    addr: String,
}

// Implementation for Server
impl Server {

    // Create new instance of server
    pub fn new(addr: String) -> Self {
        Self {
            addr,
        }
    }

    pub fn run(self) {
        println!("Listening on {} 👂🏼", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // 1024 one kilobyte is enough for a test.
                    let mut buffer = [0; 1024];
                    // convert buffer byte array in to a request.
                    match stream.read(&mut buffer) {
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
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {

                                },
                                Err(e) => println!("Failed to parse a request {}", e)
                            }
                        }
                        Err(e) => println!("Failed to read from connection {}", e)
                    }
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



