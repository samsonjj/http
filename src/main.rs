use http::{HttpServer};
use http::request::{HttpResponse, HttpStatusCode};
use std::collections::HashMap;

fn main() -> std::io::Result<()> {

    let mut server = HttpServer::new();

    server.set_handler(move |req| {
        let mut headers = HashMap::new();
        let body_bytes = b"<h1>Big boy time</h1>";
        let len = body_bytes.len();
        headers.insert("content-length".to_string(), len.to_string());

        HttpResponse {
            status_code: HttpStatusCode(200),
            http_version: "HTTP/1.1".to_string(),
            headers,
            body: Some(body_bytes.to_vec())
        }
    });

    server.listen(8080);

    Ok(())
}