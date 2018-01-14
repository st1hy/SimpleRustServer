extern crate hyper;
extern crate futures;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate time;

mod fileserver;
mod internaldata;

use futures::future::Future;
use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode};
use fileserver::Fileserver;
use std::rc::Rc;

type Server = internaldata::Server;
type InternalData = internaldata::InternalData;

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();
        debug!("New request {:?}", req);
        info!("New request {}", req.path());

        match (req.method(), req.path()) {
            (&Method::Get, "/echo") => {
                response.set_body("Try POSTing data to /echo");
                response_ok(response)
            }
            (&Method::Post, "/echo") => {
                response
                    //.with_header(ContentLength(body.len() as u64))
                    .set_body(req.body());
                response_ok(response)
            }
            (&Method::Get, "/timestamp/PHONE")  => {
                let timestamp = time::now_utc().to_timespec().sec;
                response.set_body(format!("{}", timestamp));
                response_ok(response)
            }
            (&Method::Get, "/")  => {
                self.simple_file_send("/index.html")
            }
            (&Method::Get, _) => {
                self.simple_file_send(req.path())
            }
            _ => {
                response.set_status(StatusCode::NotFound);
                response_ok(response)
            }
        }
    }
}

fn response_ok(response: Response) -> Box<Future<Item=Response, Error=hyper::Error>> {
    Box::new(futures::future::ok(response))
}


fn main() {
    env_logger::init();

    let address = "127.0.0.1:8080";
    let internaldata = InternalData::new(address, "www");
    info!("Started server listening at {}", &address);
    let addr = address.parse().unwrap();
    let rc_data = Rc::new(internaldata);

    let server = Http::new().bind(&addr, move || {
        let data_rc = rc_data.clone();
        let data = data_rc.as_ref().to_owned();
        let server = Server::new(data);
        Ok(server)
    }).unwrap();
    server.run().unwrap();
}
