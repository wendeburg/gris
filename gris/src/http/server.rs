use tokio::io;
use tokio::net::{TcpListener};
use crate::http::{Request, Response};
use crate::routing::{RouteHandlerReturnType, Router};

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

    pub async fn start(&mut self) -> io::Result<()> {
        let listener = TcpListener::bind(self.address.clone()).await?;

        match listener.accept().await {
            Ok((socket, addr)) => println!("new client: {:?}", addr),
            Err(e) => println!("couldn't get client: {:?}", e),
        }

        Ok(())
    }

    pub fn get<F>(&mut self, path: &str, f: F)
    where
        F: Fn(&'static mut Request, &'static mut Response) -> RouteHandlerReturnType,
    {

    }

    pub fn post<F>(&mut self, path: &str, f: F)
        where
            F: Fn(&'static mut Request, &'static mut Response) -> RouteHandlerReturnType,
    {

    }

    pub fn put<F>(&mut self, path: &str, f: F)
        where
            F: Fn(&'static mut Request, &'static mut Response) -> RouteHandlerReturnType,
    {

    }

    pub fn delete<F>(&mut self, path: &str, f: F)
        where
            F: Fn(&'static mut Request, &'static mut Response) -> RouteHandlerReturnType,
    {

    }
}