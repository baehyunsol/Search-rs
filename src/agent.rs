use crate::cache::LRU;
use crate::chunk::{CHUNK_SIZE, CHUNK_OVERLAP};
use crate::file::read_bytes;
use crate::hash::{hash_at_3, hash_at_5, search_at_3, search_at_5};
use crate::index::file::FileIndex;
use crate::index::hash::generate_rev_table_from_file_index;
use crate::misc::{intersect, remove_duplicate};
use hserde::HSerde;
use std::collections::{HashMap, HashSet};

pub struct Agent {
    file_index: FileIndex,
    file_cache: LRU<String, Vec<u8>, 32>,
    db: sled::Db
}

// TODO FIXME
// for now, it cannot spawn multiple agents because sled doesn't let multiple readers to access a DB
const NUM_OF_WORKERS: usize = 1;
const MOD_3: u32 = 104857;
const MOD_5: u32 = 1677721;

impl Agent {

    // if file_index already exists, it's overwritten
    // todo: use explicit error type
    pub fn init_new(path: String) -> Result<Self, ()> {
        let file_index = FileIndex::init_dir(path)?;
        let file_cache = LRU::new();

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
        let file_cache = LRU::new();

        let db = match sled::open(file_index.db_path.clone()) {
            Ok(d) => d,
            _ => { return Err(()); }
        };

        Ok(Agent { file_index, file_cache, db })
    }

    // it's `&mut self` because it might mutate its cache
    pub fn search(&mut self, keyword: &[u8]) -> Vec<(String, usize)> {  // Vec<(FileName, index)>

        let keyword_hashes_3 = if keyword.len() >= 3 {
            remove_duplicate(vec![
                hash_at_3(keyword, 0) % MOD_3,
                hash_at_3(keyword, (keyword.len() / 2).min(keyword.len() - 3)) % MOD_3,
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
                hash_at_5(keyword, (keyword.len() / 2).min(keyword.len() - 5)) % MOD_5,
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
                            panic!("todo: alert that there's an DBIOError")
                        }
                    },
                    None => {}
                },
                Err(_) => {
                    panic!("todo: alert that there's an DBIOError")
                }
            }

        }

        // HashMap<FileName, Vec<ChunkIndex>>
        let mut chunks_map: HashMap<String, Vec<usize>> = HashMap::new();

        for chunk in chunks_to_see.into_iter() {
            let (file_name, chunk_index) = self.file_index.from_chunk_index(chunk);

            match chunks_map.get_mut(&file_name) {
                Some(c) => {
                    c.push(chunk_index);
                }
                None => {
                    // TODO: my original code looked like below, which caused an error, write ODAB for that
                    // chunks_map.insert(file_name, vec![]);
                    chunks_map.insert(file_name, vec![chunk_index]);
                }
            }

        }

        let mut result: HashMap<String, HashSet<usize>> = HashMap::new();

        for (file_name, chunk_indexes) in chunks_map.into_iter() {
            let mut data: &[u8] = match self.file_cache.get(&file_name) {
                Some(data) => &data,
                None => match read_bytes(&file_name) {
                    Ok(data) => {
                        self.file_cache.put(file_name.clone(), data.clone());
                        &[]
                    },
                    Err(_) => {
                        continue;
                    }
                }
            };

            // some boilerplates to avoid the borrow checker
            if data.len() == 0 {
                data = &self.file_cache.get(&file_name).unwrap();
            }

            if keyword.len() >= 5 {

                for chunk_index in chunk_indexes.into_iter() {

                    for result_this_chunk in search_at_5(&data[(chunk_index * (CHUNK_SIZE - CHUNK_OVERLAP))..(chunk_index * (CHUNK_SIZE - CHUNK_OVERLAP) + CHUNK_SIZE).min(data.len())], 0, keyword).into_iter() {

                        match result.get_mut(&file_name) {
                            Some(indexes) => {
                                indexes.insert(result_this_chunk + chunk_index * (CHUNK_SIZE - CHUNK_OVERLAP));
                            }
                            None => {
                                let mut new_hash_set = HashSet::new();
                                new_hash_set.insert(result_this_chunk + chunk_index * (CHUNK_SIZE - CHUNK_OVERLAP));
                                result.insert(file_name.clone(), new_hash_set);
                            }
                        }

                    }

                }

            }

            else {

                for chunk_index in chunk_indexes.into_iter() {

                    for result_this_chunk in search_at_3(&data[(chunk_index * (CHUNK_SIZE - CHUNK_OVERLAP))..(chunk_index * (CHUNK_SIZE - CHUNK_OVERLAP) + CHUNK_SIZE).min(data.len())], 0, keyword).into_iter() {

                        match result.get_mut(&file_name) {
                            Some(indexes) => {
                                indexes.insert(result_this_chunk + chunk_index * (CHUNK_SIZE - CHUNK_OVERLAP));
                            }
                            None => {
                                let mut new_hash_set = HashSet::new();
                                new_hash_set.insert(result_this_chunk + chunk_index * (CHUNK_SIZE - CHUNK_OVERLAP));
                                result.insert(file_name.clone(), new_hash_set);
                            }
                        }

                    }

                }

            }

        }

        result.into_iter().map(
            |(file_name, indexes)|
            indexes.into_iter().map(
                |index|
                (file_name.clone(), index)
            ).collect()
        ).collect::<Vec<Vec<(String, usize)>>>().concat()
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