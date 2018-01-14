extern crate hyper;
extern crate futures;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate time;

mod fileserver;
mod internaldata;
mod filecache;
mod json;
mod volatiledata;
mod server;

use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode};
use fileserver::Fileserver;
use std::rc::Rc;
use internaldata::{Server, InternalData};
use volatiledata::VolatileData;
use server::{EchoHandler, BoxFuture};

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = BoxFuture;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();
        match (req.method(), req.path()) {
            (&Method::Get, "/echo") => {
                response.set_body("Try POSTing data to /echo");
                response_ok(response)
            }
            (&Method::Post, "/echo") => {
                info!("incoming post");
                self.handle_post(req)
            }
            (&Method::Get, "/timestamp/PHONE")  => {
                let timestamp = time::now_utc().to_timespec().sec;
                response.set_body(format!("{}", timestamp));
                response_ok(response)
            }
            (&Method::Get, "/")  => {
                self.maybe_cached_file_send("/index.html")
            }
            (&Method::Get, _) => {
                self.maybe_cached_file_send(req.path())
            }
            _ => {
                response.set_status(StatusCode::NotFound);
                response_ok(response)
            }
        }
    }
}

fn response_ok(response: Response) -> BoxFuture {
    Box::new(futures::future::ok(response))
}

fn main() {
    env_logger::init();
    let internaldata = InternalData::from_file(internaldata::DEFAULT_CONFIGURATION_FILE);
    let address = internaldata.server_address.clone();
    info!("Started server listening at {}", &address);
    let addr = address.parse().unwrap();
    let rc_data = Rc::new(internaldata);

    let server = Http::new().bind(&addr, move || {
        let data_rc = rc_data.clone();
        let data = data_rc.as_ref().to_owned();
        let volatile_data = VolatileData::from_file(&data.volatile_file);
        let server = Server::new(data, volatile_data);
        Ok(server)
    }).unwrap();
    server.run().unwrap();
}
