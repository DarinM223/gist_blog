use futures::future;
use futures::{Future, Stream};
use github;
use hyper::{Client, Error, Get, Uri};
use hyper::client::Request;
use hyper::header::UserAgent;
use hyper_tls::HttpsConnector;
use models;
use serde_json;
use std::io;
use std::rc::Rc;

/// Struct representing the parameters for a gist to publish.
#[derive(Deserialize, Debug)]
pub struct PublishRequest {
    pub username: String,
    pub title: String,
    pub gistid: String,
}

/// Retrieves the gist from the request parameters.
pub fn get_gist<'a>(
    params: PublishRequest,
    client: Rc<Client<HttpsConnector>>,
) -> Box<Future<Item = models::Gist, Error = Error>> {
    let url = format!("https://api.github.com/gists/{}", params.gistid)
        .parse::<Uri>()
        .unwrap();
    let mut req = Request::new(Get, url);
    req.headers_mut().set(UserAgent("gist_blog".to_string()));

    let fut = client.request(req)
        // Collect request body into byte array.
        .and_then(|res| {
            res.body().fold(vec![], |mut acc, chunk| {
                acc.extend_from_slice(&chunk);
                Ok::<_, Error>(acc)
            })
        })
        // Convert body into JSON, concat all the file markdown contents, and return a Gist.
        .and_then(move |body| {
            let body = String::from_utf8_lossy(&body[..]);
            let gist: github::Gist = match serde_json::from_str(&body) {
                Ok(gist) => gist,
                Err(_) => {
                    let io_err = io::Error::new(io::ErrorKind::InvalidData, "Error parsing JSON");
                    return future::err(Error::Io(io_err));
                }
            };

            let mut concat_body = String::new();
            for (_, ref file) in &gist.files {
                concat_body.push_str(file.content.as_str());
            }

            let new_gist = models::Gist {
                id: params.gistid,
                user_id: params.username,
                title: params.title,
                body: concat_body,
            };

            future::ok(new_gist)
        });

    Box::new(fut)
}
