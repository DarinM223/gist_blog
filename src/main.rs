#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate tera;

extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde_json;
extern crate tokio_core;

pub mod github;
pub mod handlers;
pub mod service;
pub mod models;
pub mod schema;
pub mod utils;

use hyper::server::Http;
use service::GistBlog;
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;
use futures::Stream;

fn main() {
    let addr = "127.0.0.1:1337".parse().unwrap();
    let http = Http::new();

    let mut lp = Core::new().unwrap();
    let handle = lp.handle();
    let cloned_handle = handle.clone();
    let listener = TcpListener::bind(&addr, &lp.handle()).unwrap();
    println!("Listening on http://{} with 1 thread",
             listener.local_addr().unwrap());

    let service_factory = move || GistBlog::new(cloned_handle.clone());
    let server = listener
        .incoming()
        .for_each(move |(socket, addr)| {
                      http.bind_connection(&handle, socket, addr, service_factory());
                      Ok(())
                  });

    lp.run(server).unwrap();
}
