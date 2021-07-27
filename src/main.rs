use http::*;
use std::path::PathBuf;
use std::fs;

fn main() -> std::io::Result<()> {

    let mut server = HttpServer::new();

    server.request_handler = move |req| -> HttpResponse {
        let body = b"<h1>Big boy time</h1>".to_vec();
        let not_found_body = b"<h1>404 Not Found</h1>".to_vec();

        let path = std::path::PathBuf::from(req.uri);

        if path == PathBuf::new().join("/") {
            HttpResponse::new(
                HttpVersion::default(),
                HttpStatusCode(200),
                HttpHeaders::default(),
                Some(body)
            )
        } else if path == PathBuf::new().join("/home") {
            let page = fs::read(PathBuf::from("pages/home.html")).unwrap();
            HttpResponse::new(
                HttpVersion::default(),
                HttpStatusCode(200),
                HttpHeaders::default(),
                Some(page)
            )
        } else {
            HttpResponse::new(
                HttpVersion::default(),
                HttpStatusCode(404),
                HttpHeaders::default(),
                Some(not_found_body)
            )
        }
    };

    server.listen(8080).unwrap();

    Ok(())
}