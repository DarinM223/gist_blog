use futures::future;
use futures::Future;
use futures::future::FutureResult;

use hyper;
use hyper::Client;
use hyper::StatusCode;
use hyper::header::ContentLength;
use hyper::server::Response;

use service::{Context, USER_ROUTE_MATCH};
use tera;

pub fn handle_root(service_context: &Context)
                   -> Box<Future<Item = Response, Error = hyper::Error>> {
    let mut context = tera::Context::new();
    context.add("text", &"Hello world!".to_string());
    let body = service_context.tera.borrow_mut().render("index.html", context).unwrap();

    let url = hyper::Url::parse("http://www.google.com").unwrap();
    Box::new(Client::new(&service_context.handle)
        .get(url)
        .and_then(|res| {
            future::ok(Response::new()
                .with_header(ContentLength(body.len() as u64))
                .with_body(body))
        }))
}

pub fn handle_user(service_context: &Context,
                   path: String)
                   -> Box<Future<Item = Response, Error = hyper::Error>> {
    let name = path.trim_left_matches(USER_ROUTE_MATCH).to_string();
    // TODO(DarinM223): retrieve user data from name

    let mut context = tera::Context::new();
    context.add("username", &name);
    let body = service_context.tera.borrow_mut().render("user.html", context).unwrap();

    future::ok(Response::new()
            .with_header(ContentLength(body.len() as u64))
            .with_body(body))
        .boxed()
}

pub fn handle_publish(_: &Context) -> Box<Future<Item = Response, Error = hyper::Error>> {
    // TODO(DarinM223): retrieve gist from blog
    future::ok(Response::new().with_status(StatusCode::Ok)).boxed()
}

pub fn handle_not_found(_: &Context) -> Box<Future<Item = Response, Error = hyper::Error>> {
    future::ok(Response::new().with_status(StatusCode::NotFound)).boxed()
}
