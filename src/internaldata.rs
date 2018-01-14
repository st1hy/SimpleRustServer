extern crate serde;
extern crate serde_json;

use std::io::BufWriter;
use std::path::Path;
use std::fs::{File, OpenOptions};

#[derive(Debug)]
pub struct Server {
    internals: Box<InternalData>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalData {
    pub server_address: String,
    pub files_dir: String,
}

impl InternalData {
    pub fn new(address: &str, files_dir: &str) -> InternalData {
        InternalData {
            server_address: address.to_string(),
            files_dir: files_dir.to_string(),
        }
    }

    pub fn save_to_json(&self, f: &str) {
        let path = Path::new(f);
        let mut options = OpenOptions::new();
        options.write(true)
            .truncate(true)
            .create(true);
        let file = match options.open(&path) {
            Ok(file) => file,
            Err(_) => {
                error!("Cannot open file for write {:?}", path);
                return
            },
        };
        let writer = BufWriter::new(&file);
        match serde_json::to_writer(writer, self) {
            Ok(_) => info!("Saved to json {:?}, content: {:?}", path, self),
            Err(err) => error!("save_to_json: error {}, {:?}", err.to_string(), path),
        }
    }
}

impl Server {

    pub fn new(data: InternalData) -> Server {
        Server {
            internals: Box::new(data)
        }
    }

    pub fn files_dir(&self) -> String {
        self.internals.files_dir.clone()
    }
}