use crossbeam_channel::{Receiver, Sender};
use crossbeam_channel::{bounded, unbounded};
use std::{thread, time::Duration};

use crate::{MusicPlayer, Request};

pub enum Queue {
    Add(String, u32),
    Remove(usize),
    Clear,
}
pub struct UpdateQueue;
pub fn create_queue(
    s_mp: Sender<MusicPlayer>,
    r_req: Receiver<Request>,
) -> (Sender<Queue>, Receiver<UpdateQueue>) {
    let (s, r) = unbounded();
    let (s_update, r_update) = bounded::<UpdateQueue>(1);
    let s_clone = s.clone();
    let r_update_clone = r_update.clone();

    thread::spawn(move || {
        let mut songs: Vec<String> = Vec::new();
        let mut lengths: Vec<u32> = Vec::new();
        loop {
            let received = r.recv_timeout(Duration::from_millis(10)).ok();
            if let Some(message) = received {
                match message {
                    Queue::Add(song, length) => {
                        songs.push(song);
                        lengths.push(length);
                    }
                    Queue::Remove(index) => {
                        songs.remove(index);
                        lengths.remove(index);
                    }
                    Queue::Clear => {
                        songs.clear();
                        lengths.clear();
                    }
                }
            }
            if r_req.recv_timeout(Duration::from_millis(10)).is_ok() {
                if let Some(song) = songs.first() {
                    if let Some(length) = lengths.first() {
                        s_mp.send(MusicPlayer::NewSong(song.clone(), length.to_owned()))
                            .expect("Cant send song");
                        s_update.send(UpdateQueue).expect("Cant update queue");
                        songs.remove(0);
                    }
                }
            }
        }
    });
    (s_clone, r_update_clone)
}
