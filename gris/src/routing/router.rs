use std::collections::HashMap;
use crate::http::{HttpMethod, Request, Response};
use crate::routing::path_tree::PathTree;
use crate::routing::{RouteHandlerReturnType};

pub struct Router {
    routes: HashMap<HttpMethod, PathTree>
}

impl Router {
    pub fn new() -> Router {
        return Router {
            routes: HashMap::new()
        }
    }

    pub fn add_route(&mut self, method: HttpMethod, path: &str, f: impl Fn(&'static mut Request, &'static mut Response) -> RouteHandlerReturnType + 'static) {
        let routes_of_methods = self.routes.entry(method).or_insert(PathTree::new());
        routes_of_methods.insert(path, f);
    }
}