use std::net::{TcpListener, TcpStream};
use std::thread;

pub mod parser;
pub mod request;
pub mod stream;

use stream::HttpStream;
use request::{HttpRequest, HttpResponse, HttpStatusCode, HttpVersion};

pub struct HttpServer {
    listening: bool,
    pub multi_threaded: bool,
    pub request_handler: fn(HttpRequest) -> HttpResponse,
}

use crate::request::HttpHeaders;

impl HttpServer {
    pub fn new() -> Self {
        Self {
            listening: false,
            multi_threaded: false,
            request_handler: Self::default_request_handler,
        }
    }

    fn default_request_handler(_req: HttpRequest) -> HttpResponse {
        let body_bytes = b"<h1>Hello, World!</h1>";

        return HttpResponse::new(
            HttpVersion::default(),
            HttpStatusCode(200),
            HttpHeaders::default(),
            Some(body_bytes.to_vec())
        )
    }

    fn connection_handler(stream: TcpStream, handler: fn(HttpRequest) -> HttpResponse) -> std::io::Result<()> {
        let mut stream: HttpStream<TcpStream> = HttpStream::new(stream);

        let request = stream.read_http().unwrap();
        let response = handler(request);

        stream.write(String::from(response).as_bytes());
        Ok(())
    }

    pub fn listen(&mut self, port: usize) -> Result<(), ()> {
        // can only listen once
        if self.listening { return Err(()); }
        self.listening = true;

        // let connection_handler = Self::connection_handler;
        let request_handler = self.request_handler;

        // accept connections on infinite loop
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

        for stream in listener.incoming() {
            if self.multi_threaded {
                thread::spawn(move || {
                    Self::connection_handler(stream.unwrap(), request_handler).unwrap();
                });
            } else {
                Self::connection_handler(stream.unwrap(), request_handler).unwrap();
            }
        }

        Ok(())
    }
}

/// tests: test using threads, so that we can send network requests while listening for network
/// requests!
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // let port = 8080;

        // start the server
        // listen(port);

        // execute requests against the server
        // let stream = TcpStream::connect(8080);
        // let message = b"Hello, World!";
        // stream.write(message).unwrap();
    }
}
