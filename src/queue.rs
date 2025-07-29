use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use crate::musicplayer::MusicPlayer;

pub enum Queue {
    Push(String),
    Remove(usize),
    Clear,
}

pub fn create_queue(mp: Sender<MusicPlayer>) -> Sender<Queue> {
    let (tx, rx) = mpsc::channel::<Queue>();
    let tx_clone= tx.clone();

    thread::spawn(move || {
        let mut songs: Vec<String> = Vec::new();
        loop {
            let recieved = rx.recv().unwrap();
            match recieved {
                Queue::Push(song) => {
                    songs.push(song)
                },
                Queue::Remove(index) => {
                    songs.remove(index);
                },
                Queue::Clear => {
                    songs.clear();
                }
            }
        }
    });
    tx_clone
}