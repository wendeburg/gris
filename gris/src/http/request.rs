use std::collections::HashMap;
use super::HttpMethod;

pub struct Request {
    headers: HashMap<String, Vec<u8>>,
    version: Option<u8>,
    path: Option<String>,
    body: Option<Box<[u8]>>,
    method: Option<HttpMethod>
}

impl Request {
    fn new() -> Request {
        Request {
            headers: HashMap::new(),
            version: None,
            path: None,
            method: None,
            body: None
        }
    }
}

trait RequestFromHttparseRequest {
    fn to_gris_request(self) -> Request;
}

impl RequestFromHttparseRequest for httparse::Request<'_, '_> {
    fn to_gris_request(self) -> Request {
        let mut new_request = Request::new();

        if let Some(method) = self.method {
            new_request.method = HttpMethod::from_str(method);
        }

        new_request.version = self.version;

        if let Some(path) = self.path {
            new_request.path = Some(path.to_owned());
        }

        for header in self.headers {
            new_request.headers.insert(header.name.to_owned(), header.value.to_owned());
        }

        return new_request;
    }
}