use std::sync::{mpsc};
use std::sync::mpsc::{Sender};
use std::thread;
use crate::musicplayer::MusicPlayer;


pub enum Queue {
    Add(String),
    End
}

pub enum VisualQueue {
    Add(String),
    Remove(usize),
    Clear,
    SongEnded(bool),
    End
}

pub fn create_queue(mp: Sender<MusicPlayer>) -> Sender<Queue> {
    let (tx, rx) = mpsc::channel::<Queue>();
    let tx_clone= tx.clone();

    thread::spawn(move || {
        loop {
            let recieved = rx.recv().unwrap();
            match recieved {
                Queue::Add(song) => {
                    mp.send(MusicPlayer::AddToQueue(song)).expect("Cant send song to music player thread");
                },
                Queue::End => {
                    break
                }
            }
        }
    });
    tx_clone
}

pub fn create_visual_queue(q: Sender<Queue>) -> Sender<VisualQueue> {
    let (tx, rx) = mpsc::channel::<VisualQueue>();
    let tx_clone = tx.clone();

    thread::spawn(move || {
        let mut songs: Vec<String> = Vec::new();
        loop {
            let recieved = rx.recv().unwrap();
            match recieved {
                VisualQueue::Add(song) => {
                    songs.push(song);
                },
                VisualQueue::Remove(index) => {
                    songs.remove(index);
                },
                VisualQueue::Clear => {
                    songs.clear()
                },
                VisualQueue::SongEnded(ended) => {
                    if songs.len() != 0 {
                        q.send(Queue::Add(songs.first().unwrap().to_string())).expect("Cant add song to queue");
                        songs.remove(0);
                    }
                },
                VisualQueue::End => {
                    q.send(Queue::End).expect("Cant stop thread");
                    break
                }
                _ => {}
            }
        }
    });

    tx_clone
}