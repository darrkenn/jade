use crossbeam_channel::{Receiver, Sender};
use crossbeam_channel::{bounded, unbounded};
use std::{thread, time::Duration};

use crate::app::Song;
use crate::{MusicPlayer, Request};

pub enum Queue {
    Add(Song),
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
        let mut songs: Vec<Song> = Vec::new();
        loop {
            let received = r.recv_timeout(Duration::from_millis(10)).ok();
            if let Some(message) = received {
                match message {
                    Queue::Add(song) => {
                        songs.push(song);
                    }
                    Queue::Remove(index) => {
                        if index < songs.len() {
                            songs.remove(index);
                        }
                    }
                    Queue::Clear => {
                        songs.clear();
                    }
                }
            }
            if r_req.recv_timeout(Duration::from_millis(10)).is_ok() {
                if let Some(song) = songs.first() {
                    s_mp.send(MusicPlayer::NewSong(song.clone()))
                        .expect("Cant send song");
                    s_update.send(UpdateQueue).expect("Cant update queue");
                    songs.remove(0);
                }
            }
        }
    });
    (s_clone, r_update_clone)
}
