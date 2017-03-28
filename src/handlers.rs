use diesel;
use diesel::prelude::*;
use futures::{future, Stream};
use futures::Future;
use hyper::{Error, Get, StatusCode, Uri};
use hyper::client;
use hyper::header::{ContentLength, UserAgent};
use hyper::server::{Request, Response, Service};
use hyper_tls::HttpsConnector;
use models::*;
use serde_json;
use service::{Context, GistBlog, GIST_ROUTE_MATCH, USER_ROUTE_MATCH};
use std::collections::HashMap;
use std::rc::Rc;
use tera;

type HandleFuture = <GistBlog as Service>::Future;

#[derive(Deserialize, Debug)]
pub struct PublishRequest {
    pub username: String,
    pub title: String,
    pub gistid: String,
}

#[derive(Deserialize, Debug)]
pub struct FileResponse {
    pub size: i32,
    pub raw_url: String,
    pub language: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct OwnerResponse {
    pub login: String,
    pub id: i32,
}

#[derive(Deserialize, Debug)]
pub struct GistResponse {
    pub url: String,
    pub id: String,
    pub description: String,
    pub owner: OwnerResponse,
    pub files: HashMap<String, FileResponse>,
}

pub fn get_gist<'a>(params: PublishRequest,
                    client: Rc<client::Client<HttpsConnector>>)
                    -> Box<Future<Item = Gist, Error = Error>> {
    let url = format!("https://api.github.com/gists/{}", params.gistid).parse::<Uri>().unwrap();

    let mut req = client::Request::new(Get, url);
    req.headers_mut().set(UserAgent("gist_blog".to_string()));
    let fut = client.request(req)
        .and_then(|res| {
            res.body().fold(vec![], |mut acc, chunk| {
                acc.extend_from_slice(&chunk);
                Ok::<_, Error>(acc)
            })
        })
        .and_then(move |body| {
            let body = String::from_utf8_lossy(&body[..]);
            let gist: GistResponse = serde_json::from_str(&body).unwrap();

            let mut concat_body = String::new();
            for (_, ref file) in &gist.files {
                concat_body.push_str(file.content.as_str());
            }

            let new_gist = Gist {
                id: params.gistid,
                user_id: params.username,
                title: params.title,
                body: concat_body,
            };

            future::ok(new_gist)
        });

    Box::new(fut)
}

/// Handler for GET / which should direct people to create their account.
pub fn handle_root(service_context: &Context) -> HandleFuture {
    let mut context = tera::Context::new();
    context.add("text", &"Hello world!".to_string());
    let body = service_context.tera.borrow_mut().render("index.html", context).unwrap();

    // TODO(DarinM223): render home page

    future::ok(Response::new().with_header(ContentLength(body.len() as u64)).with_body(body))
        .boxed()
}

/// Handler for GET /user/{username} that shows the user's gists.
pub fn handle_user(service_context: &Context, req: Request) -> HandleFuture {
    use schema::gists::dsl::*;

    let path = req.path().to_string();
    let name = path.trim_left_matches(USER_ROUTE_MATCH).to_string();
    let conn = service_context.pool.clone().get().unwrap();

    // TODO(DarinM223): handle error
    let results = gists.filter(user_id.eq(name.clone()))
        .limit(10)
        .load::<Gist>(&*conn)
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

/// Handler for GET /gist/{id} that shows a specific gist.
pub fn handle_gist(service_context: &Context, req: Request) -> HandleFuture {
    use schema::gists::dsl::*;

    let path = req.path().to_string();
    let name = path.trim_left_matches(GIST_ROUTE_MATCH).to_string();

    // TODO(DarinM223): retrieve gist and display in markdown view
    future::ok(Response::new().with_status(StatusCode::Ok)).boxed()
}

/// Handler for POST /publish which should publish a gist for a user.
pub fn handle_publish(service_context: &Context, req: Request) -> HandleFuture {
    // TODO(DarinM223): handle authentication
    use schema::gists;

    let (_, _, _, _, body) = req.deconstruct();
    let client = service_context.client.clone();
    let pool = service_context.pool.clone();
    let response = body.fold(vec![], |mut acc, chunk| {
            acc.extend_from_slice(chunk.as_ref());
            Ok::<_, Error>(acc)
        })
        .and_then(move |body| {
            let body_str = String::from_utf8(body).unwrap();
            let serialized: PublishRequest = serde_json::from_str(&body_str).unwrap();
            get_gist(serialized, client)
        })
        .and_then(move |new_gist| {
            println!("Inserting: {:?}", new_gist);
            let conn = pool.get().unwrap();
            diesel::insert(&NewGist::from(&new_gist))
                .into(gists::table)
                .execute(&*conn)
                .expect("Error saving new gist");
            future::ok(Response::new().with_status(StatusCode::Ok)).boxed()
        });

    Box::new(response)
}

/// Handler for an invalid route which returns a not found status code.
pub fn handle_not_found(_: &Context) -> HandleFuture {
    future::ok(Response::new().with_status(StatusCode::NotFound)).boxed()
}
