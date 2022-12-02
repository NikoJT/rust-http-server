// Include necessary std packages and imports from the module.
// Super refers to parent module, in this case http.
use super::method::{MethodError, Method};
use std::str;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Debug, Display, Formatter};
use std::str::Utf8Error;

// Struct for Request
pub struct Request {
    path: String,
    // Rust does not have a Null, therefore it needs to represent absence of value
    // Option type which has either Some or None.
    query_string: Option<String>,
    method: Method,
}

// Seems ok, but std library has a module called convert which is dedicated for type conversions.
// impl Request {
//     fn from_byte_array(buffer: &[u8]) -> Result<Self, String> {}
// }

// TryFrom implementation for Request from array slice
// TryFrom is safe and can fail, From can not fail.
// Traits work like interface abstractions in other languages.
// TryInto trait is coming for free with TryFrom trait.
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {

        // The question mark operator in rust is used as an error propagation
        // alternative to functions that return Result OR Option types.
        // The ? operator is a shortcut cut as it reduces the amount
        // of code needed to immediately return Err or None from the
        // types Result<T, Err> or Option in a function.

        // Converting a byte slice in to a string slice
        // Sugaring syntax ? for match with or
        // or returns either an OK with value or and error which needs implemented if
        // using match expression.
        let request = str::from_utf8(buffer)?;

        // Valid code
        // match get_next_word(request) {
        //     Some((method, request)) => {},
        //     None => return Err(ParseError::InvalidRequestError)
        // }

        // ok_or transforms the option in to a result
        // If option is some it will convert to OK variant
        // Ok will wrap value of some
        // we are overriding request variable with variable shadowing
        // get_next_word returns the remaining string which is then set to the shadowing request
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequestError)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequestError)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequestError)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocolError);
        }

        let method: Method = method.parse()?;
        unimplemented!();
    }
}

// in order to call the function multiple time, it needs
// a signature of tuple to contain for example the method and the rest of the string
// next time used we pass rest of the string as a parameter
// and return the word and the rest of the string.
// Needs to be wrapped in Option if there is no next word in the string. if there is no space.
fn get_next_word(request: &str) -> Option<(&str, &str)> {

    // Loop version for iterating
    // loop {
    //     let item = iter.next();
    //     match item {
    //         Some(c) => c,
    //         None() => break,
    //     }
    // }

    // for loop syntax
    // enumerate yields index and character in every iteration
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            // some wrapping tuple with two string slices.
            // first will be the word we want
            // all strings in rust needs to be valid utf8
            // i is the current index and the index of the i
            // lowerbound of the range syntax is inclusive
            // space will be a part of the return value
            // to exclude it from the value we use &request[i + 1..]
            // however this is very very dangerous and can lead to a crash.
            // this would generate invalid utf8 and our program would crash
            // if in &request[i + 1..] next is an character longer than one byte
            // in this loop this is still safe & valid, due to the space
            // on the control flow statement that mandates the space is at the position of the
            // index i so we can safely assume it is exactly one byte in size.
            return Some((&request[..i], &request[i + 1..]))
        }
    }
    unimplemented!();
}

// Enum for parsing errors
pub enum ParseError {
    InvalidRequestError,
    InvalidEncodingError,
    InvalidProtocolError,
    InvalidMethodError,
}

// Implementation for ParseError
// message for display
impl ParseError {
    fn message(&self) -> &str {
        match self {
            ParseError::InvalidRequestError => "Invalid Request",
            ParseError::InvalidEncodingError => "Invalid Encoding",
            ParseError::InvalidProtocolError => "Invalid Protocol",
            ParseError::InvalidMethodError => "Invalid Method",
        }
    }
}

// convert method error
impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethodError
    }
}

// convert utf8 error
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncodingError
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

// Implement std Error for parse, which requires
// Debug and Display to be implemented for the ParseError
impl Error for ParseError {}