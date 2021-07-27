mod parser;
mod request;
mod stream;
mod server;

pub use request::*;
pub use parser::*;
pub use stream::*;
pub use server::*;

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
