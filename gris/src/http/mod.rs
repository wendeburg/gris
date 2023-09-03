pub use response::Response;
pub use request::Request;
pub use server::HttpServer;

mod response;
mod request;

mod server;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

impl HttpMethod {
    fn from_str(string: &str) -> Option<HttpMethod> {
        match string {
            "POST" => Some(HttpMethod::POST),
            "GET" => Some(HttpMethod::GET),
            "DELETE" => Some(HttpMethod::DELETE),
            "PUT" => Some(HttpMethod::PUT),
            "HEAD" => Some(HttpMethod::HEAD),
            "CONNECT" => Some(HttpMethod::CONNECT),
            "OPTIONS" => Some(HttpMethod::OPTIONS),
            "TRACE" => Some(HttpMethod::TRACE),
            "PATCH" => Some(HttpMethod::PATCH),
            _ => None
        }
    }
}