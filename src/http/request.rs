use super::method::{METHOD, MethodError}; // super means the parent module which is http here.
use std::convert::TryFrom;
use std::convert::From;
use std::error::Error; // trait
// use std::fmt::Display; // trait
// use std::fmt::Debug; // Trait
use std::fmt::{Result as FmtResult, Display, Debug, Formatter}; // Alias for this type.
// use std::fmt::Formatter;

use std::str;
use std::str::Utf8Error;


pub struct Request {
    path: String,
    query_string: Option<String>,
    method: METHOD,
}

impl Request {
    // Result
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {

    }
}

// to implement traits - this are like interfaces in other languages
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;
    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {

        // match str::from_utf8(buf) {
        //     Ok(request) => {},
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }

        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {},
        //     Err(e) => return Err(e),
        // }

        // the ? below is taking place of the match above. Returns an error is it happens, otherwise we get the value
        // let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
        let request = str::from_utf8(buf)?;
        // match get_next_word(request) {
        //     Some((method, request)) => {},
        //     None => return Err(ParseError::InvalidRequest),
        // }

        // .ok_or Transforms the Option<T> into a [Result<T, E>], mapping Some(v) to Ok(v) and None to Err(err)
        // Here we are overwriting the request variable.
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: METHOD = method.parse()?;

        // let mut query_string = None;
        // match path.find('?') {
        //     Some(i) => {
        //         query_string = Some(&path[i + 1..]); // we are expecting there to be a ? so +1 byte is fine here
        //         path = &path[..i];
        //     },
        //     None => {}
        // }
        //
        // let q = path.find('?');
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_string = Some(&path[i + 1..]); // we are expecting there to be a ? so +1 byte is fine here
        //     path = &path[..i];
        // }
        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(&path[i + 1..]); // we are expecting there to be a ? so +1 byte is fine here
            path = &path[..i];
        }
        unimplemented!();
    }
}

// Return Option so that we can return nothing if no string is passed in.
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // Returns an iterator over the chars of a string slice. "abc d" => "a b c  d"
    // let mut chars = request.chars();
    // loop {
    //     let item = chars.next();
    //     match item {
    //         Some(c) => {},
    //         None => break,
    //     }
    // }

    for (i, c) in request.chars().enumerate(){
        if c == ' ' || c == '\r' {
            // return Some((&request[..i], &request[i+1..])); Can't use this because if a utf8 char is more than one byte we will crash
            return Some((&request[..i], &request[i+1..])); // same code but we know we can use this because we are
            // ending on a space, and we know spaces are always 1 byte.
        }
    }
    None
}
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}



impl ParseError {
    fn message(&self) -> &str {
        // The match strings are automatically returned.
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
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
impl Error for ParseError {}

// We need this here so that we can call str::from_utf8(buf)?; the ? requires this trait be implemented.
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}
