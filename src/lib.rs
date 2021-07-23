use std::net::{TcpListener, TcpStream};
use std::thread;

pub mod parser;
pub mod request;
pub mod stream;

use stream::HttpStream;
use request::{HttpRequest, HttpResponse, HttpStatusCode};

pub struct HttpServer {
    listening: bool,
    handler: fn(HttpRequest) -> HttpResponse,
}

use std::collections::HashMap;

impl HttpServer {
    pub fn new() -> Self {
        Self {
            listening: false,
            handler: HttpServer::default_handler,
        }
    }

    pub fn set_handler(&mut self, handler: fn(HttpRequest) -> HttpResponse) {
        self.handler = handler;
    }

    fn default_handler(req: HttpRequest) -> HttpResponse {
        let mut headers = HashMap::new();
        let body_bytes = b"<h1>Hello, World!</h1>";
        let len = body_bytes.len();
        headers.insert("content-length".to_string(), len.to_string());

        return HttpResponse {
            status_code: HttpStatusCode(200),
            http_version: "HTTP/1.1".to_string(),
            headers,
            body: Some(body_bytes.to_vec())
        }
    }

    fn handler(stream: TcpStream, handler: fn(HttpRequest) -> HttpResponse) -> std::io::Result<()> {
        let mut stream: HttpStream<TcpStream> = HttpStream::new(stream);

        let request = stream.read_http().unwrap();
        let response = handler(request);

        stream.write(String::from(response).as_bytes());
        Ok(())
    }

    pub fn listen(&mut self, port: usize) -> Result<(), ()> {
        if self.listening { return Err(()); }

        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

        let f = Self::handler;
        let f2 = self.handler;

        // accept connections
        for stream in listener.incoming() {
            println!("handing off");
            thread::spawn(move || { f(stream.unwrap(), f2); });
        }

        self.listening = true;
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
