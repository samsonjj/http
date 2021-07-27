use std::collections::HashMap;

pub use method::*;
pub use status::*;
pub use request::*;
pub use response::*;
pub use headers::*;

mod method {
    pub const BODIED_METHODS: [HttpMethod; 3] = [
        HttpMethod::POST,
        HttpMethod::PUT,
        HttpMethod::PATCH,
    ];

    /// HTTP Methods as defined in at <https://www.w3.org/Protocols/rfc2616/rfc2616-sec9.html>
    #[allow(dead_code)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum HttpMethod {
        OPTIONS,
        GET,
        HEAD,
        POST,
        PUT,
        DELETE,
        TRACE,
        CONNECT,
        PATCH
    }

    impl HttpMethod {
        pub fn as_str(&self) -> &str {
            match self {
                HttpMethod::OPTIONS => "OPTIONS",
                HttpMethod::GET => "GET",
                HttpMethod::HEAD => "HEAD",
                HttpMethod::POST => "POST",
                HttpMethod::PUT => "POST",
                HttpMethod::DELETE => "POST",
                HttpMethod::TRACE => "TRACE",
                HttpMethod::CONNECT => "CONNECT",
                HttpMethod::PATCH => "PATCH",
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
}

mod status {
    #[allow(dead_code)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct HttpStatusCode(pub i32);

    #[allow(dead_code)]
    impl HttpStatusCode {
        pub fn description(&self) -> &str {
            match self.0 {
                // TODO: more statuses
                100 => "Continue",
                101 => "Switching Protocols",
                102 => "Processing",
                200 => "OK",
                201 => "Created",
                202 => "Accepted",
                203 => "Non-authoritative Information",
                204 => "No Content",
                205 => "Reset Content",
                206 => "Partial Content",
                207 => "Multi-Status",
                208 => "Already Reported",
                300 => "Multiple Choices",
                301 => "Moved Permanently",
                302 => "Found",
                303 => "See Other",
                304 => "Not Modified",
                305 => "Use Proxy",
                307 => "Temporary Redirect",
                308 => "Permanent Redirect",
                400 => "Bad Request",
                401 => "Unauthorized",
                402 => "Payment Required",
                403 => "Forbidden",
                404 => "Not Found",
                405 => "Status Not Implemented",
                406 => "Status Not Implemented",
                407 => "Status Not Implemented",
                408 => "Status Not Implemented",
                409 => "Status Not Implemented",
                410 => "Status Not Implemented",
                411 => "Status Not Implemented",
                412 => "Status Not Implemented",
                413 => "Status Not Implemented",
                414 => "Status Not Implemented",
                415 => "Status Not Implemented",
                416 => "Status Not Implemented",
                417 => "Status Not Implemented",
                418 => "Status Not Implemented",
                419 => "Status Not Implemented",
                420 => "Status Not Implemented",
                421 => "Status Not Implemented",
                422 => "Status Not Implemented",
                423 => "Status Not Implemented",
                424 => "Status Not Implemented",
                425 => "Status Not Implemented",
                426 => "Status Not Implemented",
                427 => "Status Not Implemented",
                428 => "Status Not Implemented",
                429 => "Status Not Implemented",
                430 => "Status Not Implemented",
                431 => "Status Not Implemented",
                444 => "Status Not Implemented",
                451 => "Status Not Implemented",
                499 => "Status Not Implemented",
                500 => "Status Not Implemented",
                501 => "Status Not Implemented",
                502 => "Status Not Implemented",
                503 => "Status Not Implemented",
                504 => "Status Not Implemented",
                505 => "Status Not Implemented",
                506 => "Status Not Implemented",
                507 => "Status Not Implemented",
                508 => "Status Not Implemented",
                509 => "Status Not Implemented",
                510 => "Status Not Implemented",
                511 => "Status Not Implemented",
                599 => "Status Not Implemented",
                _ => panic!("invalid http status")
            }
        }
    }
}

mod request {
    use super::HttpMethod;
    use std::collections::HashMap;
    use std::path::PathBuf;

    /// An HTTP request struct. Most operations on this struct are read-only,
    /// an instance of this struct will be read in from an HttpStream and used
    /// to generate an HttpResponse.
    #[derive(Debug, Clone, PartialEq)]
    pub struct HttpRequest {
        pub method: HttpMethod,
        pub uri: PathBuf,
        pub http_version: String,
        pub headers: HashMap<String, String>,
        pub body: Option<Vec<u8>>,
    }

    impl From<HttpRequest> for String {
        fn from(req: HttpRequest) -> Self {
            let header_list: Vec<String> = req.headers
                .into_iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect();

            let body_str = match req.body {
                Some(data) => format!("\r\n{}", String::from_utf8_lossy(&data)),
                None => "".to_string()
            };

            format!("{} {} {}\r\n{}\r\n{}",
                    req.method.as_str(),
                    req.uri.to_str().unwrap(),
                    req.http_version,
                    header_list.join("\r\n"),
                    body_str
            )
        }
    }
}

#[derive(Clone, Debug)]
pub struct HttpVersion(pub String);

impl HttpVersion {
    pub fn default() -> Self {
        Self("HTTP/1.1".to_string())
    }
}

mod headers {
    use super::*;

    /// Represents a collection of HTTP headers. Handles setting header keys to lowercase.
    #[derive(Clone, Debug)]
    pub struct HttpHeaders(pub HashMap<String, String>);

    #[allow(dead_code)]
    impl HttpHeaders {
        pub fn new() -> Self {
            HttpHeaders(HashMap::new())
        }

        pub fn insert(&mut self, key: &str, value: &str) {
            self.0.insert(key.to_lowercase(), value.to_string());
        }

        pub fn get(&self, key: &str) -> Option<&String> {
            self.0.get(&key.to_lowercase())
        }

        pub fn contains_key(&mut self, key: &str) -> bool {
            self.0.contains_key(&key.to_lowercase())
        }

        pub fn unset(&mut self, key: &str) -> Option<String> {
            self.0.remove(&key.to_lowercase())
        }

        pub fn default() -> Self {
            let mut result = Self::new();
            result.insert("accept", "application/json");
            result
        }
    }

    impl From<HashMap<String, String>> for HttpHeaders {
        fn from(hm: HashMap<String, String>) -> HttpHeaders {
            let mut result = HttpHeaders::new();
            hm.iter().for_each(|(k, v)| { result.insert(&k.to_lowercase(), v); });
            result
        }
    }
}

mod response {
    use super::*;

    /// An HTTP response.
    #[derive(Debug, Clone)]
    pub struct HttpResponse {
        pub http_version: HttpVersion,
        pub status_code: HttpStatusCode,
        pub headers: HttpHeaders,
        body: Option<Vec<u8>>
    }

    impl HttpResponse {
        pub fn new(
            http_version: HttpVersion,
            status_code: HttpStatusCode,
            headers: HttpHeaders,
            body: Option<Vec<u8>>
        ) -> Self {
            let mut result = Self {
                http_version,
                status_code,
                headers,
                body: None
            };
            result.set_body(body);
            result
        }

        pub fn get_body(&self) -> &Option<Vec<u8>> {
            &self.body
        }

        // TODO: headers should be locked into only lowercase inmemory, otherwise lookups with caps will fail
        // implement newtype

        /// Sets the body field. Automatically sets the `Content-Length` header.
        pub fn set_body(&mut self, body: Option<Vec<u8>>) {
            if let Some(data) = &body {
                self.headers.insert("content-length", &data.len().to_string());
            }
            self.body = body;
        }
    }

    impl From<HttpResponse> for String {
        fn from(res: HttpResponse) -> Self {
            let header_list: Vec<String> = res.headers.0
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect();

            let body_str: String = match res.get_body() {
                Some(body ) => String::from_utf8_lossy(&body).to_string(),
                None => "".to_string()
            };

            format!("{} {} {}\r\n{}\r\n\r\n{}",
                    res.http_version.0.as_str(),
                    res.status_code.0.to_string(),
                    res.status_code.description(),
                    header_list.join("\r\n"),
                    body_str
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn string_from_request() {
        let expected = "GET /logo.gif HTTP/1.1\r\ncontent-length: 13\r\n\r\nHello, World!";

        let mut headers = HashMap::new();
        headers.insert("content-length".to_string(), "13".to_string());

        let request = HttpRequest {
            method: HttpMethod::GET,
            uri: PathBuf::from("/logo.gif"),
            http_version: String::from("HTTP/1.1"),
            headers,
            body: Some("Hello, World!".into())
        };

        assert_eq!(String::from(request), expected);
    }

    #[test]
    fn http_headers() {
        let mut headers = HttpHeaders::new();
        headers.insert("Content-Length", "14");
        assert_eq!(headers.get("Content-Length"), Some(&"14".to_string()));
        assert_eq!(headers.get("content-length"), Some(&"14".to_string()));

        headers.insert("content-length", "14");
        assert_eq!(headers.get("Content-Length"), Some(&"14".to_string()));
        assert_eq!(headers.get("content-length"), Some(&"14".to_string()));

        assert!(headers.contains_key("Content-Length"));
        assert!(headers.contains_key("content-length"));

        let mut hm: HashMap<String, String> = HashMap::new();
        hm.insert("Content-Length".to_string(), "14".to_string());

        let mut headers = HttpHeaders::from(hm);
        assert_eq!(headers.get("Content-Length"), Some(&"14".to_string()));
        assert_eq!(headers.get("content-length"), Some(&"14".to_string()));

        headers.insert("content-length", "14");
        assert_eq!(headers.get("Content-Length"), Some(&"14".to_string()));
        assert_eq!(headers.get("content-length"), Some(&"14".to_string()));

        assert!(headers.contains_key("Content-Length"));
        assert!(headers.contains_key("content-length"));
    }
}