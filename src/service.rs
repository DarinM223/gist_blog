use diesel::Connection;
use diesel::sqlite::SqliteConnection;
use futures::Future;
use handlers;
use hyper;
use hyper::{Get, Post};
use hyper::client::{Client, HttpConnector};
use hyper::server::{Service, Request, Response};
use std::cell::RefCell;
use std::rc::Rc;
use tera::Tera;
use tokio_core::reactor::Handle;

pub const USER_ROUTE_MATCH: &'static str = "/user/";
pub const DATABASE_URI: &'static str = "test.sqlite";

pub struct Context {
    pub tera: Rc<RefCell<Tera>>,
    pub conn: SqliteConnection,
    pub client: Client<HttpConnector>,
}

impl Context {
    pub fn new(handle: Handle) -> Context {
        let tera = compile_templates!("templates/**/*");
        let conn = SqliteConnection::establish(&DATABASE_URI)
            .expect(&format!("Error connecting to {}", DATABASE_URI));
        let client = Client::new(&handle);

        Context {
            tera: Rc::new(RefCell::new(tera)),
            conn: conn,
            client: client,
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
