use std::collections::HashMap;
use crate::http::HttpMethod;
use crate::routing::Callable;
use crate::routing::path_tree::PathTree;

pub struct Router {
    routes: HashMap<HttpMethod, PathTree>
}

impl Router {
    pub fn new() -> Router {
        return Router {
            routes: HashMap::new()
        }
    }

    pub fn add_route(&mut self, method: HttpMethod, path: &str, handler: impl Callable + 'static) {
        let routes_of_methods = self.routes.entry(method).or_insert(PathTree::new());
        routes_of_methods.insert(path, handler);
    }
}