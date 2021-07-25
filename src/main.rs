use http::{HttpServer};
use http::request::{HttpResponse, HttpStatusCode, HttpHeaders, HttpVersion};

fn main() -> std::io::Result<()> {

    let mut server = HttpServer::new();

    server.request_handler = move |_req| {
        let body_bytes = b"<h1>Big boy time</h1>";

        HttpResponse::new(
            HttpVersion::default(),
            HttpStatusCode(200),
            HttpHeaders::default(),
            Some(body_bytes.to_vec())
        )
    };

    server.listen(8080).unwrap();

    Ok(())
}