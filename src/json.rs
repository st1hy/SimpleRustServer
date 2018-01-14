extern crate serde;
extern crate serde_json;

use std::io::{BufWriter, BufReader};
use std::path::Path;
use std::fs::{File, OpenOptions};

pub fn read_json_file<T, F>(f: &str, fallback: F) -> T where
    T: serde::de::DeserializeOwned,
    F: Fn() -> T {
    let file = match File::open(f) {
        Ok(f) => f,
        Err(_) => {
            warn!("Cannot open json file: {}", f);
            return fallback();
        }
    };
    let reader = BufReader::new(&file);
    let data: T = match serde_json::from_reader(reader) {
        Ok(data) => data,
        Err(_) => {
            warn!("Cannot parse configuration file {}", f);
            return fallback();
        }
    };
    data
}

pub fn write_json_file<T>(t: &T, f: &str) where T: serde::ser::Serialize {
    let path = Path::new(f);
    let mut options = OpenOptions::new();
    options.write(true)
        .truncate(true)
        .create(true);
    let file = match options.open(&path) {
        Ok(file) => file,
        Err(_) => {
            error!("Cannot open file for write {:?}", path);
            return;
        }
    };
    let writer = BufWriter::new(&file);
    match serde_json::to_writer_pretty(writer, t) {
        Ok(_) => info!("Saved to json {:?}", path),
        Err(err) => error!("save_to_json: error {}, {:?}", err.to_string(), path),
    }
}