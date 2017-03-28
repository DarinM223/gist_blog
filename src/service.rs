use diesel::sqlite::SqliteConnection;
use futures::Future;
use handlers;
use hyper;
use hyper::{Get, Post};
use hyper::client::Client;
use hyper::server::{Service, Request, Response};
use hyper_tls::HttpsConnector;
use r2d2::{Config, Pool};
use r2d2_diesel::ConnectionManager;
use std::cell::RefCell;
use std::rc::Rc;
use tera::Tera;
use tokio_core::reactor::Handle;

pub const USER_ROUTE_MATCH: &'static str = "/user/";
pub const GIST_ROUTE_MATCH: &'static str = "/gist/";
pub const DATABASE_URI: &'static str = "test.sqlite";

pub struct Context {
    pub tera: Rc<RefCell<Tera>>,
    pub pool: Pool<ConnectionManager<SqliteConnection>>,
    pub client: Rc<Client<HttpsConnector>>,
}

impl Context {
    pub fn new(handle: Handle) -> Context {
        let tera = compile_templates!("templates/**/*");
        let config = Config::default();
        let manager = ConnectionManager::<SqliteConnection>::new(DATABASE_URI);
        let pool = Pool::new(config, manager).expect("Failed to create database pool");
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle)) // Allow https:// requests.
            .build(&handle);

        Context {
            tera: Rc::new(RefCell::new(tera)),
            pool: pool,
            client: Rc::new(client),
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
        let path = req.path().to_string();
        let method = req.method().clone();
        match (method, path.as_str()) {
            (Get, "/") => handlers::handle_root(&self.context),
            (Get, path) if path.starts_with(USER_ROUTE_MATCH) => {
                handlers::handle_user(&self.context, req)
            }
            (Get, path) if path.starts_with(GIST_ROUTE_MATCH) => {
                handlers::handle_gist(&self.context, req)
            }
            (Post, "/publish") => handlers::handle_publish(&self.context, req),
            _ => handlers::handle_not_found(&self.context),
        }
    }
}
