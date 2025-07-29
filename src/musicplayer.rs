use std::fs::File;
use std::io::BufReader;
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;
use rodio::{Decoder, Sink, Source};
use crate::Jade;

pub enum MusicPlayer {
    Pause,
    Stop,
    Volume(f32),
    NewSong(String),
    AddToQueue(String),
    End
}

pub fn create_mp(volume: f32) -> Sender<MusicPlayer> {
    let (tx, rx) = mpsc::channel::<MusicPlayer>();
    let tx_clone = tx.clone();
    thread::spawn(move || {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream().expect("Cant open stream");
        let sink = Sink::connect_new(&stream_handle.mixer());
        let mut current_duration: u64 = 0;
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
                    sink.clear();
                },
                MusicPlayer::Volume(volume) => {
                    //This seems really stupid but it works
                    if volume == 0.0 {
                        sink.set_volume(0.0);
                    } else {
                        sink.set_volume(volume);
                        let current_pos = sink.get_pos().as_secs_f64().round() as u64;
                    }
                },
                MusicPlayer::NewSong(song) => {
                    sink.clear();
                    let song = create_song(song);
                    let duration = song.total_duration().unwrap().as_secs_f64().round() as u64;
                    current_duration = duration;
                    sink.append(song);
                    sink.play();

                },
                MusicPlayer::AddToQueue(song) => {
                    sink.append(create_song(song));
                },

                MusicPlayer::End => {
                    break;
                }
            }
        };
    });
    tx_clone
}

fn create_song(song: String) -> Decoder<BufReader<File>> {
    let file = BufReader::new(File::open(&song).unwrap_or_else(|_| panic!("Cant read file: {}", song)));
    Decoder::new(file).unwrap()
}