extern crate futures;
extern crate hyper;

use internaldata::Server;

use futures::future::Future;
use futures::{Stream};
use futures::future;
use hyper::header::ContentLength;
use hyper::server::{Request, Response};

pub trait EchoHandler {
    fn handle_post(&self, req: Request) -> Box<Future<Item = Response, Error = hyper::Error>>;
}

impl EchoHandler for Server {

    fn handle_post(&self, req: Request) -> Box<Future<Item = Response, Error = hyper::Error>> {
        let res = Response::new();
        let (_method, _uri, _version, headers, body) = req.deconstruct();
        let input: Vec<u8> = if let Some(len) = headers.get::<ContentLength>() {
            Vec::with_capacity(**len as usize)
        } else {
            Vec::new()
        };
        Box::new(future::result(body.fold(input, |mut acc, chunk| {
            acc.extend_from_slice(chunk.as_ref());
            Ok::<_, hyper::Error>(acc)
        }).and_then(move |body_vec| {
            debug!("body_vec ready: {}", body_vec.len());
            future::ok(res.with_body(body_vec))
        }).wait()))
    }
}