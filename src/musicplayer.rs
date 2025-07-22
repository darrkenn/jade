use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use rodio::{Decoder, Sink};

pub enum MusicPlayer {
    Pause,
    Stop,
    Volume(f32),
    NewSong(String),
}

pub fn create_mp(volume: f32) -> Sender<MusicPlayer> {
    let (tx, rx) = mpsc::channel::<MusicPlayer>();

    thread::spawn(move || {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream().expect("Cant open stream");
        let sink = Sink::connect_new(&stream_handle.mixer());
        sink.set_volume(volume);
        loop {
            let received = rx.recv().unwrap();
            match received {
                MusicPlayer::Pause => {
                    if sink.is_paused() {
                        sink.play()
                    } else {
                        sink.pause()
                    }
                },
                MusicPlayer::Stop => {
                    sink.clear()
                },
                MusicPlayer::Volume(volume) => {
                    //This seems really stupid but it works
                    if volume == 0.0 {
                        sink.set_volume(0.0);
                    } else {
                        sink.set_volume(volume);
                    }
                },
                MusicPlayer::NewSong(song) => {
                    sink.clear();
                    let file = BufReader::new(File::open(&song).unwrap_or_else(|_| panic!("Cant read file: {}", song)));
                    let decoded_file = Decoder::new(file).unwrap();
                    sink.append(decoded_file);
                    sink.play();
                }
            }
        };
    });
    tx.clone()
}