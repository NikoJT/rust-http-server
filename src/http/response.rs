use std::io::{Write, Result as IoResult};
use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }    
    // Static dispatch with &mut impl Write
    // resolved at compile time. compile will resolve all concrete implementations
    // we will use. It will look every parameter that can be passed to this function.
    // Throughout this codebase the compiler will look at the type that we're calling this 
    // function with and for every different concrete type were calling the function with.
    // the compiler will copy the function and type it with the type its called.
    // at compile time compiler will see that were calling it with TcpStream
    // And will generate a function that is populated with the used type
    // This will reduce runtime cost and overhead and there wont be need for a V table
    // Main downside of this however is that compiler will take longer to compile 
    // as it needs more code to be generated and a larger binary.
    // MIGHT BE ISSUE FOR EMBEDDED. But not for web development or 
    // development on applications.
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {} \r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
