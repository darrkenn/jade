use std::thread;

use crossbeam_channel::{Receiver, Sender};

use crate::musicplayer::{MusicPlayer, Request};

pub enum Queue {
    Add(String),
    Remove(usize),
    Clear,
}
pub fn create_queue(s_mp: Sender<MusicPlayer>, r_req: Receiver<Request>) -> Sender<Queue> {
    let (s, r) = crossbeam_channel::unbounded();
    let s_clone = s.clone();

    thread::spawn(move || {
        let mut songs: Vec<String> = Vec::new();
        loop {
            let received = r.try_recv().ok();
            if let Some(message) = received {
                match message {
                    Queue::Add(song) => {
                        songs.push(song);
                    }
                    Queue::Remove(index) => {
                        songs.remove(index);
                    }
                    Queue::Clear => {
                        songs.clear();
                    }
                }
            }
            if r_req.try_recv().is_ok() {
                if let Some(song) = songs.first() {
                    s_mp.send(MusicPlayer::NewSong(song.clone()))
                        .expect("Cant send song");
                    songs.remove(0);
                }
            }
        }
    });
    s_clone
}
