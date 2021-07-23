
use std::net::{TcpStream};
use std::io::{Write, BufReader};
use std::collections::HashMap;

use crate::parser::HttpParser;

pub const BODIED_METHODS: [HttpMethod; 3] = [
    HttpMethod::POST,
    HttpMethod::PUT,
    HttpMethod::PATCH,
];

/// HTTP Methods as defined in at [https://www.w3.org/Protocols/rfc2616/rfc2616-sec9.html]
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum HttpMethod {
    OPTIONS,
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    TRACE,
    CONNECT,
    PATCH
}

impl HttpMethod {
    pub fn as_str(&self) -> &str {
        match self {
            HttpMethod::OPTIONS => "OPTIONS",
            HttpMethod::GET => "GET",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "POST",
            HttpMethod::DELETE => "POST",
            HttpMethod::TRACE => "TRACE",
            HttpMethod::CONNECT => "CONNECT",
            HttpMethod::PATCH => "PATCH",
        }
    }
}

impl std::convert::TryFrom<&str> for HttpMethod {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            _ => Err(format!("Invalid HttpMethod: {}", value))
        }
    }
}

/// Wraps std::net::TcpStream with functionality to read/write structured http requests/responses.
pub struct HttpStream {
    stream: TcpStream,
    has_been_read: bool
}

impl HttpStream {
    pub fn new(stream: TcpStream) -> Self {
        HttpStream {
            stream,
            has_been_read: false
        }
    }

    pub fn read_http(&mut self) -> Result<Request, ()> {
        if self.has_been_read {
            Err(())
        } else {
            self.has_been_read = true;
            Ok(HttpParser::new(BufReader::new(&self.stream)).parse_http_request())
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        self.stream.write(data).unwrap();
    }
}

#[allow(dead_code)]
pub struct HttpStatus(i32);

#[allow(dead_code)]
impl HttpStatus {
    pub fn as_str(&self) -> &str {
        match self.0 {
            // TODO: more statuses
            100 => "Continue",
            101 => "Switching Protocols",
            102 => "Processing",
            200 => "OK",
            201 => "Created",
            202 => "Accepted",
            203 => "Non-authoritative Information",
            204 => "No Content",
            205 => "Reset Content",
            206 => "Partial Content",
            207 => "Multi-Status",
            208 => "Already Reported",
            300 => "Multiple Choices",
            301 => "Moved Permanently",
            302 => "Found",
            303 => "See Other",
            304 => "Not Modified",
            305 => "Use Proxy",
            307 => "Temporary Redirect",
            308 => "Permanent Redirect",
            400 => "Bad Request",
            401 => "Unauthorized",
            402 => "Payment Required",
            403 => "Forbidden",
            404 => "Status Not Implemented",
            405 => "Status Not Implemented",
            406 => "Status Not Implemented",
            407 => "Status Not Implemented",
            408 => "Status Not Implemented",
            409 => "Status Not Implemented",
            410 => "Status Not Implemented",
            411 => "Status Not Implemented",
            412 => "Status Not Implemented",
            413 => "Status Not Implemented",
            414 => "Status Not Implemented",
            415 => "Status Not Implemented",
            416 => "Status Not Implemented",
            417 => "Status Not Implemented",
            418 => "Status Not Implemented",
            419 => "Status Not Implemented",
            420 => "Status Not Implemented",
            421 => "Status Not Implemented",
            422 => "Status Not Implemented",
            423 => "Status Not Implemented",
            424 => "Status Not Implemented",
            425 => "Status Not Implemented",
            426 => "Status Not Implemented",
            427 => "Status Not Implemented",
            428 => "Status Not Implemented",
            429 => "Status Not Implemented",
            430 => "Status Not Implemented",
            431 => "Status Not Implemented",
            444 => "Status Not Implemented",
            451 => "Status Not Implemented",
            499 => "Status Not Implemented",
            500 => "Status Not Implemented",
            501 => "Status Not Implemented",
            502 => "Status Not Implemented",
            503 => "Status Not Implemented",
            504 => "Status Not Implemented",
            505 => "Status Not Implemented",
            506 => "Status Not Implemented",
            507 => "Status Not Implemented",
            508 => "Status Not Implemented",
            509 => "Status Not Implemented",
            510 => "Status Not Implemented",
            511 => "Status Not Implemented",
            599 => "Status Not Implemented",
            _ => panic!("invalid http status")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub method: HttpMethod,
    pub uri: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }
}