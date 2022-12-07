use crate::hash::{hash_at_3, hash_at_5};
use crate::index::file::FileIndex;
use crate::index::hash::generate_rev_table_from_file_index;
use crate::misc::{intersect, remove_duplicate};
use hserde::HSerde;
use std::collections::HashMap;

pub struct Agent {
    file_index: FileIndex,
    file_cache: HashMap<String, Vec<u8>>,
    db: sled::Db
}

const CACHE_SIZE: usize = 32;
const NUM_OF_WORKERS: usize = 4;
const MOD_3: u32 = 104857;
const MOD_5: u32 = 1677721;

impl Agent {

    // if file_index already exists, it's overwritten
    // todo: use explicit error type
    pub fn init_new(path: String) -> Result<Self, ()> {
        let file_index = FileIndex::init_dir(path)?;
        let file_cache = HashMap::with_capacity(CACHE_SIZE);

        // Todo: make params configurable
        generate_rev_table_from_file_index(
            &file_index,
            NUM_OF_WORKERS,
            MOD_3,
            MOD_5
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
                hash_at_3(keyword, 0) % MOD_3,
                hash_at_3(keyword, keyword.len() / 2) % MOD_3,
                hash_at_3(keyword, keyword.len() - 3) % MOD_3
            ])
        }

        // the keyword is too short
        else {
            return vec![];
        };

        let keyword_hashes_5 = if keyword.len() >= 5 {
            remove_duplicate(vec![
                hash_at_5(keyword, 0) % MOD_5,
                hash_at_5(keyword, keyword.len() / 2) % MOD_5,
                hash_at_5(keyword, keyword.len() - 5) % MOD_5
            ])
        }

        else {
            vec![]
        };

        let hashes_to_see = remove_duplicate(vec![keyword_hashes_3, keyword_hashes_5].concat());
        let mut chunks_to_see: Vec<u64> = vec![];

        // for now, hash_3 and hash_5 are stored in the same DB
        for hash in hashes_to_see.into_iter() {

            match self.db.get(&hash.to_bytes()) {
                Ok(d) => match d {
                    Some(dd) => match Vec::<u64>::from_bytes(&dd, 0) {
                        Ok(v) => {

                            if chunks_to_see.len() == 0 {
                                chunks_to_see = v.clone();
                            }

                            else {
                                chunks_to_see = intersect(chunks_to_see, v);
                            }

                        },
                        Err(_) => {
                            // todo: alert that there's an DBIOError
                        }
                    },
                    None => {}
                },
                Err(_) => {
                    // todo: alert that there's an DBIOError
                }
            }

        }

        // todo: chunks를 같은 파일들끼리 묶고 각각 검색!

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