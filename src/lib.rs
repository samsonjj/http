use std::net::{TcpListener, TcpStream};

mod request;
mod parser;

use request::HttpStream;

pub fn handle_client(stream: TcpStream) {
    let mut stream = HttpStream::new(stream);

    let request = stream.read_http().unwrap();

    stream.write(b"HTTP/1.1 200 OK\r\n\r\n");

    println!("method: {}", request.method.as_str());
    println!("headers: {:?}", request.headers);
    if let Some(data) = request.body {
        println!("body: {:?}", String::from_utf8_lossy(&data));
    } else {
        println!("body: <none>");
    }
}

pub fn listen(port: i32) -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

    // accept connections
    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
    Ok(())
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
