#![deny(warnings)]
extern crate futures;
extern crate hyper;

use futures::{Future, Sink};
use futures::sync::{mpsc, oneshot};

use hyper::{Chunk, StatusCode};
use hyper::error::Error;
use hyper::header::ContentLength;
use hyper::server::Response;

use std::fs::File;
use std::io::{self, copy, Read};
use std::thread;

use internaldata;


static MSG_FILE_NOT_FOUND: &'static str = "File not Found";


pub trait Fileserver {
    fn simple_file_send(&self, f: &str) -> Box<Future<Item = Response, Error = hyper::Error>>;
    fn stream_file(&self, f: &str) -> Box<Future<Item=Response, Error=hyper::Error>>;
    fn get_file_path(&self, f: &str) -> String;
}

impl Fileserver for internaldata::Server {

    fn get_file_path(&self, f: &str) -> String {
        let mut filename = self.files_dir();
        filename = filename + f;
        filename
    }
    // Serve a file by reading it entirely into memory. As a result
    // this is limited to serving small files, but it is somewhat
    // simpler with a little less overhead.
    //
    // On channel errors, we panic with the expect method. The thread
    // ends at that point in any case.
    fn simple_file_send(&self, f: &str) -> Box<Future<Item = Response, Error = hyper::Error>> {

        let filename = self.get_file_path(f);
        let (tx, rx) = oneshot::channel();
        thread::spawn(move || {
            let mut file = match File::open(filename) {
                Ok(f) => f,
                Err(_) => {
                    tx.send(Response::new()
                        .with_status(StatusCode::NotFound)
                        .with_header(ContentLength(MSG_FILE_NOT_FOUND.len() as u64))
                        .with_body(MSG_FILE_NOT_FOUND))
                        .expect("Send error on open");
                    return;
                },
            };
            let mut buf: Vec<u8> = Vec::new();
            match copy(&mut file, &mut buf) {
                Ok(_) => {
                    let res = Response::new()
                        .with_header(ContentLength(buf.len() as u64))
                        .with_body(buf);
                    tx.send(res).expect("Send error on successful file read");
                },
                Err(_) => {
                    tx.send(Response::new().with_status(StatusCode::InternalServerError)).
                        expect("Send error on error reading file");
                },
            };
        });

        Box::new(rx.map_err(|e| Error::from(io::Error::new(io::ErrorKind::Other, e))))
    }

    fn stream_file(&self, f: &str) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let filename = self.get_file_path(f);
        let (tx, rx) = oneshot::channel();
        thread::spawn(move || {
            let mut file = match File::open(filename) {
                Ok(f) => f,
                Err(_) => {
                    let body = MSG_FILE_NOT_FOUND;
                    tx.send(Response::new()
                        .with_status(StatusCode::NotFound)
                        .with_header(ContentLength(body.len() as u64))
                        .with_body(body))
                        .expect("Send error on open");
                    return;
                }
            };
            let (mut tx_body, rx_body) = mpsc::channel(1);
            let res = Response::new().with_body(rx_body);
            tx.send(res).expect("Send error on successful file read");
            let mut buf = [0u8; 4096];
            loop {
                match file.read(&mut buf) {
                    Ok(n) => {
                        if n == 0 {
                            // eof
                            tx_body.close().expect("panic closing");
                            break;
                        } else {
                            let chunk: Chunk = buf.to_vec().into();
                            match tx_body.send(Ok(chunk)).wait() {
                                Ok(t) => { tx_body = t; }
                                Err(_) => { break; }
                            };
                        }
                    }
                    Err(_) => { break; }
                }
            }
        });
        Box::new(rx.map_err(|e| Error::from(io::Error::new(io::ErrorKind::Other, e))))
    }
}
