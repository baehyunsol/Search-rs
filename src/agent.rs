use crate::hash::{hash_at_3, hash_at_5};
use crate::index::file::FileIndex;
use crate::index::hash::generate_rev_table_from_file_index;
use crate::misc::remove_duplicate;
use std::collections::HashMap;

pub struct Agent {
    file_index: FileIndex,
    file_cache: HashMap<String, Vec<u8>>,
    db: sled::Db
}

const CACHE_SIZE: usize = 32;

impl Agent {

    // if file_index already exists, it's overwritten
    // todo: use explicit error type
    pub fn init_new(path: String) -> Result<Self, ()> {
        let file_index = FileIndex::init_dir(path)?;
        let file_cache = HashMap::with_capacity(CACHE_SIZE);

        // Todo: make params configurable
        generate_rev_table_from_file_index(
            &file_index,
            4,       // num of workers
            104857,  // mod_3
            1677721  // mod_5
        );

        let db = match sled::open(file_index.db_path.clone()) {
            Ok(d) => d,
            _ => { return Err(()); }
        };

        Ok(Agent { file_index, file_cache, db })
    }

    // returns Err if no file_index exists at the given path
    pub fn load_new(path: String) -> Result<Self, ()> {
        let file_index = FileIndex::read_dir(path)?;
        let file_cache = HashMap::with_capacity(CACHE_SIZE);

        let db = match sled::open(file_index.db_path.clone()) {
            Ok(d) => d,
            _ => { return Err(()); }
        };

        Ok(Agent { file_index, file_cache, db })
    }

    pub fn search(&self, keyword: &[u8]) -> Vec<(String, usize)> {  // Vec<(FileName, index)>

        let keyword_hashes_3 = if keyword.len() >= 3 {
            remove_duplicate(vec![
                hash_at_3(keyword, 0),
                hash_at_3(keyword, keyword.len() / 2),
                hash_at_3(keyword, keyword.len() - 3)
            ])
        }

        // the keyword is too short
        else {
            return vec![];
        };

        let keyword_hashes_5 = if keyword.len() >= 5 {
            remove_duplicate(vec![
                hash_at_5(keyword, 0),
                hash_at_5(keyword, keyword.len() / 2),
                hash_at_5(keyword, keyword.len() - 5)
            ])
        }

        else {
            vec![]
        };

        // 방금 구한 hash들을 db에 검색해보고 거기서 나오는 chunk들을 읽어서 쭉 검색!
        // db에 검색해서 나온 chunk들 remove_dupl하는 거 잊지말고! 같은 파일끼리 묶는 것도 잊지말고!

        todo!()
    }

    // Todo: which return type should it have?
    pub fn search_pipeline(&self, keyword: &[u8]) -> Vec<(String, usize)> {
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