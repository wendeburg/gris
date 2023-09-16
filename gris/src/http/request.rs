use std::collections::HashMap;
use super::HttpMethod;

pub struct Request<B> {
    headers: HashMap<String, String>,
    version: Option<u8>,
    path: Option<String>,
    body: Option<B>,
    method: Option<HttpMethod>
}

impl<B> Request<B> {
    fn new() -> Self {
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
    fn to_gris_request(self) -> Request<Vec<u8>>;
}

impl RequestFromHttparseRequest for httparse::Request<'_, '_> {
    fn to_gris_request(self) -> Request<Vec<u8>>{
        let mut new_request = Request::new();

        if let Some(method) = self.method {
            new_request.method = HttpMethod::from_str(method);
        }

        new_request.version = self.version;

        if let Some(path) = self.path {
            new_request.path = Some(path.to_owned());
        }

        for header in self.headers {
            new_request.headers.insert(header.name.to_owned(), String::from_utf8(header.value.to_vec()).unwrap());
        }

        return new_request;
    }
}