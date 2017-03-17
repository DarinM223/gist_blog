use diesel::pg::PgConnection;
use futures::Future;

use handlers;
use hyper;
use hyper::{Get, Post, Client};
use hyper::server::{Service, Request, Response};

use tera::Tera;
use tokio_core;
use tokio_core::reactor::Handle;
use super::establish_connection;
use std::cell::RefCell;
use std::rc::Rc;

pub const USER_ROUTE_MATCH: &'static str = "/user/";

pub struct Context {
    pub tera: Rc<RefCell<Tera>>,
    pub conn: PgConnection,
    pub handle: Handle,
}

impl Context {
    pub fn new(handle: Handle) -> Context {
        let tera = compile_templates!("templates/**/*");
        let conn = establish_connection();

        Context {
            tera: Rc::new(RefCell::new(tera)),
            conn: conn,
            handle: handle,
        }
    }
}

pub struct GistBlog {
    pub context: Context,
}

impl GistBlog {
    pub fn new(handle: Handle) -> GistBlog {
        GistBlog { context: Context::new(handle) }
    }
}

impl Service for GistBlog {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Response, Error = hyper::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Get, "/") => handlers::handle_root(&self.context),
            (&Get, path) if path.starts_with(USER_ROUTE_MATCH) => {
                handlers::handle_user(&self.context, path.to_string())
            }
            (&Post, "/publish") => handlers::handle_publish(&self.context),
            _ => handlers::handle_not_found(&self.context),
        }
    }
}
