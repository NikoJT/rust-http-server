// used to convert string to enum
use std::str::FromStr;
use std::fmt::Debug;
// Enums for Method
// Enums are just number GET = 1 and so on,
// Sugaring to hide "magical numbers"
#[derive(Debug)]
pub enum Method {
    GET,
    PUT,
    POST,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
    TRACE,
    CONNECT,
}

// Implement FromStr trait for Method
// To convert into pattern
impl FromStr for Method {
    type Err = MethodError;
    // from_str required by FormStr trait
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "PUT" => Ok(Self::PUT),
            "POST" => Ok(Self::POST),
            "PATCH" => Ok(Self::PATCH),
            "DELETE" => Ok(Self::DELETE),
            "HEAD" => Ok(Self::HEAD),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "CONNECT" => Ok(Self::CONNECT),
            _ => Err(MethodError)
        }
    }
}

pub struct MethodError;
