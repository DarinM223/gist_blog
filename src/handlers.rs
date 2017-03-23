use diesel::prelude::*;
use futures::future;
use futures::Future;
use hyper;
use hyper::StatusCode;
use hyper::header::ContentLength;
use hyper::server::Response;
use models::*;
use service::{Context, USER_ROUTE_MATCH};
use tera;

pub fn handle_root(service_context: &Context)
                   -> Box<Future<Item = Response, Error = hyper::Error>> {
    let mut context = tera::Context::new();
    context.add("text", &"Hello world!".to_string());
    let body = service_context.tera.borrow_mut().render("index.html", context).unwrap();

    // TODO(DarinM223): render home page

    future::ok(Response::new().with_header(ContentLength(body.len() as u64)).with_body(body))
        .boxed()
}

pub fn handle_user(service_context: &Context,
                   path: String)
                   -> Box<Future<Item = Response, Error = hyper::Error>> {
    use schema::gists::dsl::*;

    let name = path.trim_left_matches(USER_ROUTE_MATCH).to_string();

    // TODO(DarinM223): handle error
    let results = gists.filter(user_id.eq(name.clone()))
        .limit(10)
        .load::<Gist>(&service_context.conn)
        .expect("Error loading users gists");

    let mut context = tera::Context::new();
    context.add("username", &name);
    context.add("results", &results);
    let html_body = service_context.tera.borrow_mut().render("user.html", context).unwrap();

    future::ok(Response::new()
            .with_header(ContentLength(html_body.len() as u64))
            .with_body(html_body))
        .boxed()
}

pub fn handle_publish(_: &Context) -> Box<Future<Item = Response, Error = hyper::Error>> {
    // TODO(DarinM223): retrieve gist from blog
    future::ok(Response::new().with_status(StatusCode::Ok)).boxed()
}

pub fn handle_not_found(_: &Context) -> Box<Future<Item = Response, Error = hyper::Error>> {
    future::ok(Response::new().with_status(StatusCode::NotFound)).boxed()
}
