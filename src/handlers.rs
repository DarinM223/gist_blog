use futures::future;
use futures::future::FutureResult;

use hyper;
use hyper::StatusCode;
use hyper::header::ContentLength;
use hyper::server::Response;

use tera::Context;
use service::{GistBlog, USER_ROUTE_MATCH};

pub type Result<T> = FutureResult<T, hyper::Error>;

pub fn handle_root(service: &GistBlog) -> Result<Response> {
    let mut context = Context::new();
    context.add("text", &"Hello world!".to_string());
    let body = service.tera.borrow_mut().render("index.html", context).unwrap();

    future::ok(Response::new()
        .with_header(ContentLength(body.len() as u64))
        .with_body(body))
}

pub fn handle_user(service: &GistBlog, path: String) -> Result<Response> {
    let name = path.trim_left_matches(USER_ROUTE_MATCH).to_string();
    let mut context = Context::new();
    context.add("username", &name);
    let body = service.tera.borrow_mut().render("user.html", context).unwrap();

    future::ok(Response::new()
        .with_header(ContentLength(body.len() as u64))
        .with_body(body))
}

pub fn handle_publish(_: &GistBlog) -> Result<Response> {
    future::ok(Response::new().with_status(StatusCode::Ok))
}

pub fn handle_not_found(_: &GistBlog) -> Result<Response> {
    future::ok(Response::new().with_status(StatusCode::NotFound))
}
