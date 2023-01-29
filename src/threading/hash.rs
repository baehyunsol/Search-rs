use crate::file::read_bytes;
use crate::index::file::FileIndex;
use crate::index::hash::*;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;

pub enum MessageFromMain {
    Run {
        db: sled::Db,
        total_worker_num: usize,
        curr_worker_index: usize,
        file_index: FileIndex,
        mod_by_3: u32,
        mod_by_5: u32,
        mutex: Option<Arc<Mutex<()>>>
    },
}

pub enum MessageToMain {
    Progress (usize),
    FileNotFound (String),
    DBError (DBError)
}

pub enum DBError {
    DBOpenFailure,
    DBIOFailure(String),
    DBValuePoisoned,
    MutexPoisoned
}

pub struct Channel {
    pub tx_from_main: mpsc::Sender<MessageFromMain>,
    pub rx_to_main: mpsc::Receiver<MessageToMain>,
}

pub fn init_loop() -> Channel {
    let (tx_to_main, rx_to_main) = mpsc::channel();
    let (tx_from_main, rx_from_main) = mpsc::channel();

    thread::spawn(move || {
        event_loop(tx_to_main, rx_from_main);
    });

    Channel {
        rx_to_main, tx_from_main
    }

}

pub fn event_loop(tx_to_main: mpsc::Sender<MessageToMain>, rx_from_main: mpsc::Receiver<MessageFromMain>) {

    for msg in rx_from_main {

        match msg {
            MessageFromMain::Run { db, total_worker_num, curr_worker_index, file_index, mod_by_3, mod_by_5, mutex } => {
                let mut rev_table_3 = RevTable::with_capacity(0x8_000);
                let mut rev_table_5 = RevTable::with_capacity(0x8_000);

                // it tells the progress of this worker to the master
                let mut counter = 0;

                for (i, file_name) in file_index.files.into_iter().enumerate() {

                    if i % total_worker_num != curr_worker_index {
                        continue;
                    }

                    match read_bytes(&file_name) {
                        Ok(bytes) => {
                            let hash_3 = make_chunk_hash_3(&bytes);
                            let hash_5 = make_chunk_hash_5(&bytes);

                            update_rev_table(hash_3, i, mod_by_3, &mut rev_table_3);
                            update_rev_table(hash_5, i, mod_by_5, &mut rev_table_5);
                        },
                        Err(_) => {
                            tx_to_main.send(MessageToMain::FileNotFound(file_name.clone())).unwrap();
                        }
                    }

                    if rev_table_3.len() > 0x8_000 {
                        write_to_db(&db, rev_table_3, tx_to_main.clone(), mutex.clone());
                        rev_table_3 = RevTable::with_capacity(0x8_000);
                    }

                    if rev_table_5.len() > 0x8_000 {
                        write_to_db(&db, rev_table_5, tx_to_main.clone(), mutex.clone());
                        rev_table_5 = RevTable::with_capacity(0x8_000);
                    }

                    counter += 1;

                    if counter % 8 == 0 && counter > 0 {
                        // sending too frequently is expensive
                        tx_to_main.send(MessageToMain::Progress(counter)).unwrap();
                        counter = 0;
                    }

                }

                if rev_table_3.len() > 0 {
                    write_to_db(&db, rev_table_3, tx_to_main.clone(), mutex.clone());
                }

                if rev_table_5.len() > 0 {
                    write_to_db(&db, rev_table_5, tx_to_main.clone(), mutex.clone());
                }

                if counter > 0 {
                    tx_to_main.send(MessageToMain::Progress(counter)).unwrap();
                }

                // it runs only once
                break;
            }
        }

    }

    drop(tx_to_main)
}
