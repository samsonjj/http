use std::io::{Read, Write, BufReader};

use crate::request::{HttpRequest};
use crate::parser::HttpParser;

/// Wraps std::net::TcpStream with functionality to read/write structured http requests/responses.
pub struct HttpStream<T: Read + Write + Unpin> {
    stream: T,
    has_been_read: bool
}

impl<T: Read + Write + Unpin> HttpStream<T> {
    pub fn new(stream: T) -> Self {
        HttpStream {
            stream,
            has_been_read: false
        }
    }

    pub fn read_http(&mut self) -> Result<HttpRequest, ()> {
        if self.has_been_read {
            Err(())
        } else {
            self.has_been_read = true;
            Ok(HttpParser::new(BufReader::new(&mut self.stream)).parse_http_request())
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        self.stream.write(data).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{HttpMethod};
    use std::collections::HashMap;
    use std::path::PathBuf;

    use std::cmp::min;

    struct MockTcpStream {
        read_data: Vec<u8>,
        write_data: Vec<u8>,
    }

    impl Read for MockTcpStream {
        fn read(
            &mut self,
            buf: &mut [u8],
        ) -> std::io::Result<usize> {
            let size: usize = min(self.read_data.len(), buf.len());
            buf[..size].copy_from_slice(&self.read_data[..size]);
            Ok(size)
        }
    }

    impl Write for MockTcpStream {
        fn write(
            &mut self,
            buf: &[u8],
        ) -> std::io::Result<usize> {
            self.write_data = Vec::from(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    use std::marker::Unpin;
    impl Unpin for MockTcpStream {}

    const SIMPLE_REQUEST_STR: &str = "GET / HTTP/1.1\r\nAccept: */*\r\n\r\n";
    fn get_simple_request() -> HttpRequest {
        let mut ex_headers = HashMap::new();
        ex_headers.insert("accept".to_string(), "*/*".to_string());
        HttpRequest {
            method: HttpMethod::GET,
            uri: PathBuf::from("/"),
            http_version: "HTTP/1.1".to_string(),
            headers: ex_headers,
            body: None
        }
    }

    const BODIED_REQUEST_STR: &str = "POST / HTTP/1.1\r\nAccept: */*\r\nContent-Length: 14\r\n\r\nThis is a body\r\n\r\n";
    fn get_bodied_request() -> HttpRequest {
        let mut ex_headers = HashMap::new();
        ex_headers.insert("accept".to_string(), "*/*".to_string());
        ex_headers.insert("content-length".to_string(), "14".to_string());
        HttpRequest {
            method: HttpMethod::POST,
            uri: PathBuf::from("/"),
            http_version: "HTTP/1.1".to_string(),
            headers: ex_headers,
            body: Some(Vec::from("This is a body".as_bytes()))
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }

    fn stream_request_helper(request_str: &str, expected_request: HttpRequest) {
        let mut mock_stream = MockTcpStream {
            read_data: Vec::from(request_str.as_bytes()),
            write_data: Vec::new()
        };
        let mut http_stream = HttpStream::new(&mut mock_stream);

        let request = http_stream.read_http().unwrap();
        http_stream.write("response".as_bytes());

        assert_eq!(request, expected_request);
        assert!(mock_stream.write_data.starts_with("response".as_bytes()));
    }

    #[test]
    fn stream() {
        stream_request_helper(SIMPLE_REQUEST_STR, get_simple_request());
        stream_request_helper(BODIED_REQUEST_STR, get_bodied_request());
    }
}
