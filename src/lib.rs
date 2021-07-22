
use std::net::{TcpListener, TcpStream};
use std::io::Read;

pub fn handle_client(stream: TcpStream) {
    // let mut buffer: [u8; 1024] = [0; 1024];

    // stream.read(&mut buffer).unwrap();

    let mut stream = HttpStream(stream);
    let request = stream.read_http();

    stream.write(b"HTTP/1.1 200 OK\r\n\r\n");

    println!("method: {}", request.method.as_str());
    println!("headers: {:?}", request.headers);
    match request.body {
        Some(data) => println!("body: {}", String::from_utf8_lossy(&data)),
        None => println!("body: <none>")
    };
}

pub fn listen(port: i32) -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

    // accept connections
    for stream in listener.incoming() {
        println!("handling?");
        handle_client(stream.unwrap());
    }
    Ok(())
}

// Pattern 1:
// trait HttpStream {
//     fn read_http(&mut self) -> String;
// }
//
// impl HttpStream for TcpStream {
//     fn read_http(&mut self) -> String {
//         let mut buffer: [u8; 1024] = [0; 1024];
//
//         self.read(&mut buffer).unwrap();
//
//         String::from_utf8_lossy(&buffer[..]).into()
//     }
// }

const N: u8 = '\n' as u8;
const RN: [u8; 2] = ['\r' as u8, '\n' as u8];
const RNRN: [u8; 4] = ['\r' as u8, '\n' as u8, '\r' as u8, '\n' as u8];

#[derive(Debug, Clone, PartialEq)]
enum HttpMethod {
    GET,
    POST,
}

impl HttpMethod {
    fn as_str(&self) -> &str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST"
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

// Pattern 2:
struct HttpStream(TcpStream);

use std::io::{Write, BufReader, BufRead};
use std::convert::TryInto;

impl HttpStream {

    fn parse_request_head(&self, buffered: &mut BufReader<&TcpStream>) -> Request {
        let mut vec: Vec<u8> = vec![];

        // method
        buffered.read_until(N, &mut vec).unwrap();
        let string = String::from_utf8_lossy(&vec);
        let mut split = string.split(" ");
        let t = split.next().unwrap();
        let method: HttpMethod = t.try_into().unwrap();

        vec.clear();

        // read bytes, with \n delimiter, until we find \r\n\r\n
        let mut flag = false;
        let mut headers: HashMap<String, String> = HashMap::new();
        loop {
            buffered.read_until(N, &mut vec);

            if vec.starts_with(&RN) { break; }

            // split into two, by first `:`
            let string = String::from_utf8_lossy(&vec);
            let mut splitter = string.split(':');

            let key = splitter.next().unwrap().trim().to_lowercase();
            let value = splitter.fold("".to_string(), |a, b| a + b).trim().to_string();

            headers.insert(key.into(), value.into());

            vec.clear()
        }

        println!("after");
        Request {
            method,
            headers,
            body: None
        }
    }

    fn parse_request_body(&self, buffered: &mut BufReader<&TcpStream>, request: &mut Request) {
        let s = request.headers.get("content-length").unwrap();
        println!("content-length is:{}", s);
        let content_length: usize = request.headers.get("content-length").unwrap().parse().unwrap();
        let mut data: Vec<u8> = vec![0u8; content_length];
        buffered.read_exact(&mut data).unwrap();
        request.body = Some(data);
    }

    pub fn read_http(&mut self) -> Request {
        let n = '\n' as u8;
        let rnrn = ['\r' as u8, '\n' as u8, '\r' as u8, '\n' as u8];

        // read bytes, with \n delimiter, until we find \r\n\r\n
        // loop {
        //     buffered.read_until(n, &mut vec).unwrap();
        //     if vec.ends_with(&rnrn) { break };
        // }

        // parse method
        let mut buffered = BufReader::new(&self.0);
        let mut request = self.parse_request_head(&mut buffered);
        if [HttpMethod::POST].contains(&request.method) {
            self.parse_request_body(&mut buffered, &mut request);
        }

        request
        // let body =

        // String::from_utf8_lossy(&vec).into()
    }

    pub fn write(&mut self, data: &[u8]) {
        self.0.write(data).unwrap();
    }
}

struct HttpStatus(i32);

impl HttpStatus {
    pub fn as_str(&self) -> &str {
        match self.0 {
            // TODO: more statuses
            100 => "Continue",
            200 => "OK",
            201 => "Created",
            204 => "No Content",
            304 => "Not Modified",
            400 => "Bad Request",
            _ => panic!("invalid http status")
        }
    }
}

use std::collections::HashMap;
struct Request {
    pub method: HttpMethod,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}


/// tests: test using threads, so that we can send network requests while listening for network
/// requests!
#[cfg(test)]
mod tests {
    use std::thread;
    use super::*;

    #[test]
    fn it_works() {
        let port = 8080;

        // start the server
        listen(port);

        // execute requests against the server
        let stream = TcpStream::connect(8080);
        let message = b"Hello, World!";
        stream.write(message).unwrap();
    }
}
