use std::collections::HashMap;
use std::sync::{Arc, RwLock};

type Key = String;
type Val = Vec<u8>;
type Map = HashMap<Key, Val>;
pub type Cache = Arc<RwLock<Box<Map>>>;

#[derive(Debug)]
pub struct FileCache {
    pub cache: Cache,
}

impl FileCache {

    pub fn new() -> FileCache {
        FileCache {
            cache: Arc::new(RwLock::new(Box::new(HashMap::new()))),
        }
    }

}