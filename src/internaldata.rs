use json::{read_json_file, write_json_file};
use filecache;
use volatiledata;

pub type FileCache = filecache::FileCache;
pub type VolatileData = volatiledata::VolatileData;

#[derive(Debug)]
pub struct Server {
    pub internals: InternalData,
    pub file_cache: FileCache,
    pub volatile_data: VolatileData,
}

impl Server {
    pub fn new(data: InternalData, volatile_data: VolatileData) -> Server {
        Server {
            internals: data,
            file_cache: FileCache::new(),
            volatile_data,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalData {
    pub server_address: String,
    pub files_dir: String,
    pub volatile_file: String,
    pub use_cache: bool,
}

pub const DEFAULT_ADDRESS: &'static str = "127.0.0.1:8080";
pub const DEFAULT_FILES_DIR: &'static str = "www";
pub const DEFAULT_CONFIGURATION_FILE: &'static str = "server.json";
pub const DEFAULT_VOLATILE_FILE: &'static str = "data.json";
pub const DEFAULT_USE_CACHE: bool = true;


impl InternalData {
    pub fn from_file(f: &str) -> InternalData {
        read_json_file(&f, InternalData::defaults)
    }

    fn defaults() -> InternalData {
        let data = InternalData::new(
            DEFAULT_ADDRESS,
            DEFAULT_FILES_DIR,
            DEFAULT_USE_CACHE,
        );
        data.to_file(DEFAULT_CONFIGURATION_FILE);
        data
    }

    pub fn new(address: &str, files_dir: &str, use_cache: bool) -> InternalData {
        InternalData {
            server_address: address.to_string(),
            files_dir: files_dir.to_string(),
            volatile_file: DEFAULT_VOLATILE_FILE.to_string(),
            use_cache,
        }
    }

    pub fn to_file(&self, f: &str) {
        write_json_file(&self, f);
    }
}