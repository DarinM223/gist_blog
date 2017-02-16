use futures::future::FutureResult;

use handlers;
use hyper;
use hyper::{Get, Post};
use hyper::server::{Service, Request, Response};

use tera::Tera;
use std::cell::RefCell;
use std::rc::Rc;

pub const USER_ROUTE_MATCH: &'static str = "/user/";

pub struct GistBlog {
    pub tera: Rc<RefCell<Tera>>,
}

impl GistBlog {
    pub fn new() -> GistBlog {
        let tera = compile_templates!("templates/**/*");
        GistBlog { tera: Rc::new(RefCell::new(tera)) }
    }
}

impl Service for GistBlog {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Get, "/") => handlers::handle_root(self),
            (&Get, path) if path.starts_with(USER_ROUTE_MATCH) => {
                handlers::handle_user(self, path.to_string())
            }
            (&Post, "/publish") => handlers::handle_publish(self),
            _ => handlers::handle_not_found(self),
        }
    }
}
