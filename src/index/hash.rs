use super::file::FileIndex;
use crate::chunk::{get_chunk_index, MAX_CHUNK_COUNT, CHUNK_SIZE};
use crate::hash::{hash_at_3, hash_at_5};
use crate::threading::hash::{DBError, MessageFromMain, MessageToMain, init_loop};
use hserde::HSerde;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::{TryRecvError, Sender};

// (hash -> Vec<ChunkIndex>)
// RevTable[31] = [2, 3, 4] -> Chunk 2, Chunk 3, and Chunk 4 contains hash 31.
pub type RevTable = HashMap<u32, Vec<u64>>;

pub fn write_to_db(db: &sled::Db, rev_table: RevTable, channel: Sender<MessageToMain>) {
    // Todo: mutex when accessing DB
    // or, does sled take care of mutex internally?

    for (hash, chunks) in rev_table.into_iter() {
        let hash_bytes = hash.to_bytes();

        // old data
        let mut chunks_in_db: Vec<u64> = match db.get(&hash_bytes) {
            Ok(d) => match d {
                Some(dd) => match HSerde::from_bytes(&dd, 0) {
                    Ok(v) => v,
                    Err(_) => {
                        channel.send(MessageToMain::DBError(DBError::DBIOFailure)).unwrap();
                        vec![]
                    }
                },
                None => vec![]
            },
            Err(_) => {
                channel.send(MessageToMain::DBError(DBError::DBIOFailure)).unwrap();
                vec![]
            }
        };

        // update the old data
        for chunk in chunks.into_iter() {
            chunks_in_db.push(chunk);
        }

        match db.insert(&hash_bytes, chunks_in_db.to_bytes()) {
            Ok(_) => {}
            Err(_) => {
                channel.send(MessageToMain::DBError(DBError::DBIOFailure)).unwrap();
            }
        }
    }

}

// it initializes DB on disk
fn generate_rev_table_from_file_index(
    file_index: &FileIndex,
    total_worker_num: usize,
    mod_by_3: u32, mod_by_5: u32,
) {

    let mut channels = Vec::with_capacity(file_index.files.len());

    for i in 0..total_worker_num {
        let c = init_loop();

        // initializes the workers
        c.tx_from_main.send(MessageFromMain::Run {
            total_worker_num,
            curr_worker_index: i,
            file_index: file_index.clone(),
            mod_by_3,
            mod_by_5,
            db_path: file_index.db_path.clone()
        }).unwrap();

        channels.push(c);
    }

    while channels.len() > 0 {
        let mut disconnected_channel = Vec::with_capacity(channels.len());

        for (i, c) in channels.iter().enumerate() {
            match c.rx_to_main.try_recv() {
                Ok(m) => match m {
                    MessageToMain::Progress(p) => {
                        // Todo: record progress
                    }
                    MessageToMain::FileNotFound(f) => {
                        // Todo: handle error
                    }
                    MessageToMain::DBError(e) => match e {
                        DBError::DBOpenFailure => {
                            // Todo: handle error
                        }
                        DBError::DBIOFailure => {
                            // Todo: handle error
                        }
                    }
                }
                Err(e) => match e {
                    TryRecvError::Disconnected => {
                        disconnected_channel.push(i);
                    }
                    _ => {}
                }
            }
        }

        channels = channels.into_iter().enumerate().filter(|(i, _)| !disconnected_channel.contains(i)).map(|(_, c)| c).collect();
    }

}

// let's say the first chunk has hash 0, 1, 2, and 3, the second chunk has 4, 5, 6, and 7.
// the result would be [[0, 1, 2, 3], [4, 5, 6, 7]]
// it removes duplicates
// `bytes` is generated by file::read_bytes(file_name)
pub fn make_chunk_hash_3(bytes: &[u8]) -> Vec<HashSet<u32>> {

    if bytes.len() < 3 {
        return vec![HashSet::new()];
    }

    let chunk_indexes = get_chunk_index(bytes.len() - 2);
    let mut result = Vec::with_capacity(chunk_indexes.len());

    for chunk_index in chunk_indexes.into_iter() {
        let mut curr_set = HashSet::with_capacity(CHUNK_SIZE);
        let mut curr_hash = hash_at_3(bytes, chunk_index);

        for ind in (chunk_index + 3)..(chunk_index + CHUNK_SIZE + 3).min(bytes.len()) {
            curr_set.insert(curr_hash);
            curr_hash /= 256;
            curr_hash += bytes[ind] as u32 * 0x10_000;
        }

        curr_set.insert(curr_hash);
        result.push(curr_set);
    }

    result
}

pub fn make_chunk_hash_5(bytes: &[u8]) -> Vec<HashSet<u32>> {

    if bytes.len() < 5 {
        return vec![HashSet::new()];
    }

    let chunk_indexes = get_chunk_index(bytes.len() - 4);
    let mut result = Vec::with_capacity(chunk_indexes.len());

    for chunk_index in chunk_indexes.into_iter() {
        let mut curr_set = HashSet::with_capacity(CHUNK_SIZE);
        let mut curr_hash = hash_at_5(bytes, chunk_index);

        for ind in (chunk_index + 5)..(chunk_index + CHUNK_SIZE + 5).min(bytes.len()) {
            curr_set.insert(curr_hash);
            curr_hash /= 64;
            curr_hash += (bytes[ind] % 64) as u32 * 0x1_000_000;
        }

        curr_set.insert(curr_hash);
        result.push(curr_set);
    }

    result
}

// `chunk_hashes` is generated by `make_chunk_hash_x`.
pub fn update_rev_table(chunk_hashes: Vec<HashSet<u32>>, file_index: usize, mod_by: u32, table: &mut RevTable) {
    let mut chunk_index: u64 = file_index as u64 * MAX_CHUNK_COUNT as u64;

    for chunk_hashes in chunk_hashes.iter() {

        for hash in chunk_hashes.iter() {

            match table.get_mut(&(*hash % mod_by)) {
                None => {
                    table.insert(*hash % mod_by, vec![chunk_index]);
                }
                Some(v) => {
                    v.push(chunk_index);
                }
            }

        }

        chunk_index += 1;
    }

}

#[cfg(test)]
mod tests {
    use crate::chunk::*;
    use crate::hash::*;
    use crate::index::hash::*;
    use std::collections::HashSet;

    #[test]
    fn make_chunk_hash_test() {
        let samples: Vec<Vec<u8>> = vec![
            vec![(0..71).collect::<Vec<u8>>();71].concat(),
            vec![(0..73).collect::<Vec<u8>>();73].concat(),
            vec![(0..79).collect::<Vec<u8>>();79].concat(),
            vec![(0..83).collect::<Vec<u8>>();83].concat(),
            vec![(0..89).collect::<Vec<u8>>();89].concat(),
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 3],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4, 5],
            vec![20;6553]
        ];

        for sample in samples.into_iter() {
            let hash_3 = hash_string_3(&sample);
            let hash_5 = hash_string_5(&sample);
            let chunk_hash_3 = make_chunk_hash_3(&sample);
            let chunk_hash_5 = make_chunk_hash_5(&sample);
            let chunk_indexes = get_chunk_index(sample.len());

            // I don't want to test these cases
            if sample.len() > 4 {
                assert_eq!(get_chunk_index(sample.len()), get_chunk_index(sample.len() - 2));
                assert_eq!(get_chunk_index(sample.len()), get_chunk_index(sample.len() - 4));
            }

            for (i, chunk_i) in chunk_indexes.into_iter().enumerate() {
                let chunk_hash_vec_3 = hash_3[chunk_i..(chunk_i + CHUNK_SIZE).min(hash_3.len())].to_vec();
                assert_eq!(HashSet::<u32>::from_iter(chunk_hash_vec_3.into_iter()), chunk_hash_3[i]);

                let chunk_hash_vec_5 = hash_5[chunk_i..(chunk_i + CHUNK_SIZE).min(hash_5.len())].to_vec();
                assert_eq!(HashSet::<u32>::from_iter(chunk_hash_vec_5.into_iter()), chunk_hash_5[i]);
            }

        }

    }

}