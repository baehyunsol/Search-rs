use crate::index::file::FileIndex;
use std::collections::HashMap;

pub struct Agent {
    file_index: FileIndex,
    file_cache: HashMap<String, Vec<u8>>
}

impl Agent {

    // if file_index already exists, it's overwritten
    pub fn init_new(path: String) -> Self {
        todo!()
    }

    // returns Err if no file_index exists at the given path
    pub fn load_new(path: String) -> Result<Self, ()> {
        todo!()
    }

    pub fn search(&self, keyword: &[u8]) -> Vec<(String, usize)> {  // Vec<(FileName, index)>
        todo!()
    }

    // incrementally updates file_index and DB
    pub fn append_file(&mut self, file_name: String) -> Result<(), ()> {
        todo!()
    }

    // incrementally updates file_index and DB
    pub fn append_files(&mut self, file_names: Vec<String>) -> Result<(), ()> {
        todo!()
    }

}