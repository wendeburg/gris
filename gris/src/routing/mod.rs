use std::future::Future;
use std::pin::Pin;
pub use router::Router;
use crate::http::{Request, Response};

pub type RouteHandlerReturnType = Pin<Box<dyn Future<Output = ()> + Send>>;

mod router;
mod path_tree;

