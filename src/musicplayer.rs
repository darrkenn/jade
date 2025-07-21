use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use rodio::{Decoder, Sink};

pub enum MusicPlayer {
    Pause,
    Play,
    Stop,
    Volume(f32),
    NewSong(String),
}

pub fn create_mp() -> Sender<MusicPlayer> {
    let (tx, rx) = mpsc::channel::<MusicPlayer>();

    thread::spawn(move || {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream().expect("Cant open stream");
        let sink = Sink::connect_new(&stream_handle.mixer());
        loop {
            let received = rx.recv().unwrap();
            match received {
                MusicPlayer::Play => {
                    sink.play()
                },
                MusicPlayer::Pause => {
                    sink.pause();
                },
                MusicPlayer::Stop => {
                    sink.stop()
                },
                MusicPlayer::Volume(volume) => {
                    sink.set_volume(volume);
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