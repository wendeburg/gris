use std::collections::HashMap;

pub struct Response {
    headers: HashMap<String, String>,
    body: Vec<u8>
}