extern crate futures;
extern crate hyper;

use internaldata::Server;

use futures::future::Future;
use futures::{Stream};
use hyper::header::ContentLength;
use hyper::server::{Request, Response};

pub type BoxFuture = Box<Future<Item = Response, Error = hyper::Error>>;

pub trait EchoHandler {
    fn handle_post(&self, req: Request) -> BoxFuture;
}

impl EchoHandler for Server {

    fn handle_post(&self, req: Request) -> BoxFuture {
        let mut res = Response::new();
        let (_method, _uri, _version, headers, body) = req.deconstruct();
        let input: Vec<u8> = if let Some(len) = headers.get::<ContentLength>() {
            res.headers_mut().set(len.clone());
            Vec::with_capacity(**len as usize)
        } else {
            Vec::new()
        };
        Box::new(body.fold(input, |mut acc, chunk| {
            acc.extend_from_slice(chunk.as_ref());
            Ok::<_, hyper::Error>(acc)
        }).and_then(move |value| {
            debug!("Incoming post: {}", String::from_utf8(value.clone()).unwrap_or("error".to_string()));
            Ok(res.with_body(value))
        }))
    }
}