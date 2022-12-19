use std::fmt::{Display, Formatter, Result as FmtResult};
// derive copy, but when using copy compiler requires to implement clone trait
// if a type implements copy, it can have trivial implementation for clone that performs
// same task as copy.
// derive debug as good practices.
#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
            Self::InternalServerError => "Internal Server Error",
        }
    }
}

impl Display for StatusCode {
   fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // Must be u16 because 254 is the highest on u8
        // Were casting a reference to status code as a u16
        // as were casting pointer we will need to point the
        // actual value not the pointer so we need to de-reference the
        // reference by putting a star before the reference.
        // we want to cast whatever the self is referencing.
        // StatusCode does not implement the copy trait,
        // so the compiler cant move out of the self because of the
        // shared reference. 
        // We can think of values as having two types
        // either the ones living on the stack or on the memory heap
        // types that live entirely in the stack can be trivially copied
        // just by copying their bites, an example of this would be an integer
        // String is not a copy type. String stores on the stack only some meta-data
        // and it stores a pointer that points to the heap where the actual text is stored
        // so by copying the stack we only copy the pointer and the meta-data.
        // for complex datatypes you use clone which does deep copy that copies the necessary
        // heap data.
        // We don't have to manually implement copy we can use derive attribute with copy
        write!(f, "{}", *self as u16)
    } 
}
