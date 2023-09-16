use std::io;
use std::net::TcpListener;
use crate::http::HttpMethod;
use crate::routing::{Callable, Router};

pub struct HttpServer {
    address: String,
    router: Router
}

impl HttpServer {
    pub fn new(address: String) -> io::Result<Self> {
        return Ok(HttpServer {
            address,
            router: Router::new()
        });
    }

    pub fn start(&mut self) -> io::Result<()> {
        let listener = TcpListener::bind(self.address.clone())?;

        match listener.accept() {
            Ok((socket, addr)) => println!("new client: {:?}", addr),
            Err(e) => println!("couldn't get client: {:?}", e),
        }

        Ok(())
    }

    pub fn get(&mut self, path: &str, callable: impl Callable + 'static) {
        self.router.add_route(HttpMethod::GET, path, callable);
    }

    pub fn post(&mut self, path: &str, callable: impl Callable + 'static) {
        self.router.add_route(HttpMethod::POST, path, callable);
    }

    pub fn put(&mut self, path: &str, callable: impl Callable + 'static) {
        self.router.add_route(HttpMethod::PUT, path, callable);
    }

    pub fn delete(&mut self, path: &str, callable: impl Callable + 'static) {
        self.router.add_route(HttpMethod::DELETE, path, callable);
    }

    pub fn connect(&mut self, path: &str, callable: impl Callable + 'static) {
        self.router.add_route(HttpMethod::CONNECT, path, callable);
    }

    pub fn options(&mut self, path: &str, callable: impl Callable + 'static) {
        self.router.add_route(HttpMethod::OPTIONS, path, callable);
    }

    pub fn patch(&mut self, path: &str, callable: impl Callable + 'static) {
        self.router.add_route(HttpMethod::PATCH, path, callable);
    }

    pub fn trace(&mut self, path: &str, callable: impl Callable + 'static) {
        self.router.add_route(HttpMethod::TRACE, path, callable);
    }

    pub fn head(&mut self, path: &str, callable: impl Callable + 'static) {
        self.router.add_route(HttpMethod::HEAD, path, callable);
    }
}