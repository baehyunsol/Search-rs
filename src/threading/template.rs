use crate::file::read_bytes;
use crate::index::file::FileIndex;
use crate::index::hash::*;
use std::sync::mpsc;
use std::thread;

pub enum MessageFromMain {
    //
}

pub enum MessageToMain {
    //
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
            //
        }

    }

    drop(tx_to_main)
}
