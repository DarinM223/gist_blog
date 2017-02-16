extern crate futures;
extern crate hyper;

#[macro_use]
extern crate tera;

pub mod handlers;
pub mod service;
use hyper::server::Http;
use service::GistBlog;

fn main() {
    let addr = "127.0.0.1:1337".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(GistBlog::new())).unwrap();
    println!("Listening on http://{} with 1 thread",
             server.local_addr().unwrap());
    server.run().unwrap();
}
