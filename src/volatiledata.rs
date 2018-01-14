extern crate serde;
extern crate serde_json;

use self::serde_json::Value;
use std::sync::{Arc, RwLock};
use json::{read_json_file, write_json_file};

type Data = Arc<RwLock<Value>>;

#[derive(Debug)]
pub struct VolatileData {
    pub data: Data
}

impl VolatileData {
    pub fn from_file(f: &str) -> VolatileData {
        let value: Value = read_json_file(f, || {
            let value = json!(null);
            write_json_file(&value, &f);
            value
        });
        VolatileData {
            data: Arc::new(RwLock::new(value))
        }
    }

    pub fn to_file(&self, f: &str) {
        let d = self.data.clone();
        match d.read() {
            Ok(v) => {
                let value = v.to_owned();
                write_json_file(&value, &f);
            }
            Err(_) => {
                warn!("Volatile data lock is poisoned");
            }
        };
    }
}