use std::io::{BufRead};
use std::collections::HashMap;
use std::convert::{TryInto};

use crate::request::{Request, HttpMethod};
use crate::request;

const N: u8 = '\n' as u8;
const RN: [u8; 2] = ['\r' as u8, '\n' as u8];

/// Contains methods for parsing an HTTP request from types implementing the Read trait.
pub struct HttpParser<T: BufRead> {
    reader: T
}

impl<T: BufRead> HttpParser<T> {
    /// Gets a new HttpParser.
    pub fn new(reader: T) -> Self { HttpParser{ reader } }

    pub fn parse_http_request(&mut self) -> Request {
        let mut request = self.parse_request_head();
        if request::BODIED_METHODS.contains(&request.method) {
            self.parse_request_body(&mut request).unwrap();
        }

        request
    }

    /// private: Read and parse the head of an HTTP request from a BufReader.
    fn parse_request_head(&mut self) -> Request {
        let mut vec: Vec<u8> = vec![];

        // Parse the Request-Line
        self.reader.read_until(N, &mut vec).unwrap();
        let string = String::from_utf8_lossy(&vec);
        let mut split = string.split(" ");
        let method: HttpMethod = split.next().unwrap().trim().try_into().unwrap();
        let uri = split.next().unwrap().trim().to_string();
        let http_version = split.next().unwrap().trim().to_string();

        vec.clear();

        // read bytes, with \n delimiter, until we find \r\n\r\n
        let mut headers: HashMap<String, String> = HashMap::new();
        loop {
            self.reader.read_until(N, &mut vec).unwrap();

            if vec.starts_with(&RN) { break; }

            // split into two, by first `:`
            let string = String::from_utf8_lossy(&vec);
            let mut splitter = string.split(':');

            let key = splitter.next().unwrap().trim().to_lowercase();
            let value = splitter.fold("".to_string(), |a, b| a + b).trim().to_string();

            headers.insert(key.into(), value.into());

            vec.clear()
        }

        Request {
            method,
            uri,
            http_version,
            headers,
            body: None
        }
    }

    /// private: Read and parse an HTTP body from the BufReader and attach it to request.
    /// The request must already have headers, so that we can grab the content-length.
    fn parse_request_body(&mut self, request: &mut Request) -> std::io::Result<()> {
        let content_length: usize = request.headers.get("content-length").unwrap().parse().unwrap();

        let mut data: Vec<u8> = vec![0u8; content_length];
        self.reader.read_exact(&mut data)?;

        request.body = Some(data);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_REQUEST_STR: &str = "GET / HTTP/1.1\r\nAccept: */*\r\n\r\n";
    fn get_simple_request() -> Request {
        let mut ex_headers = HashMap::new();
        ex_headers.insert("accept".to_string(), "*/*".to_string());
        Request {
            method: HttpMethod::GET,
            uri: "/".to_string(),
            http_version: "HTTP/1.1".to_string(),
            headers: ex_headers,
            body: None
        }
    }

    #[test]
    fn parse_simple_request() {
        let mut parser: HttpParser<&[u8]> = HttpParser::new(SIMPLE_REQUEST_STR.as_bytes());
        let request = parser.parse_http_request();

        assert_eq!(request, get_simple_request());
    }
}