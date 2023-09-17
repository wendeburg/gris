use std::fmt::{Debug, Formatter};
use crate::http::{Request, Response};

pub use router::Router;

pub trait Callable {
    fn call(&mut self) -> ();
}

impl Debug for dyn Callable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Callable")
    }
}

pub struct DefaultCallable<B: 'static> {
    req: Option<Request<B>>,
    res: Option<Response>,
    function: fn(Request<B>, Response) -> ()
}

impl<B> Callable for DefaultCallable<B> {
    fn call(&mut self) -> () {
        if let None = self.req {
            panic!("Cannot call handler without Request object");
        }
        else if let None = self.res {
            panic!("Cannot call handler without Response object");
        }

        (self.function)(self.req.take().unwrap(), self.res.take().unwrap());
    }
}

mod router;
mod path_tree;

