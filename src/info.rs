use std::{thread, time::Duration};

use crossbeam_channel::{Receiver, Sender, bounded};

pub enum Info {
    Song(String, u32),
    Position(u32),
    Clear,
}

pub fn create_info() -> (Sender<Info>, Receiver<Info>) {
    let (s, r) = bounded::<Info>(2);
    let s_clone = s.clone();

    let (s_ui, r_ui) = bounded::<Info>(2);

    thread::spawn(move || {
        loop {
            let recieved = r.recv_timeout(Duration::from_millis(10)).ok();
            if let Some(message) = recieved {
                match message {
                    Info::Song(song, length) => {
                        s_ui.send(Info::Song(song, length))
                            .expect("Cant send message to UI thread. Song");
                    }
                    Info::Position(x) => {
                        s_ui.send(Info::Position(x))
                            .expect("Cant send message to UI thread Position");
                    }
                    Info::Clear => {
                        s_ui.send(Info::Clear)
                            .expect("Cant send message to UI thread CLEAR");
                    }
                }
            }
        }
    });
    (s_clone, r_ui)
}
