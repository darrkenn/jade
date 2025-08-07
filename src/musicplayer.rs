use rodio::{Decoder, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::{panic, thread};

pub enum MusicPlayer {
    Pause,
    Stop,
    Volume(f32),
    NewSong(String),
    End,
}
pub enum Queue {
    Add(String),
    Remove(usize),
    Clear,
}

pub fn create_mp(volume: f32) -> (Sender<MusicPlayer>, Sender<Queue>) {
    let (tx_mp, rx_mp) = mpsc::channel::<MusicPlayer>();
    let (tx_q, rx_q) = mpsc::channel::<Queue>();
    thread::spawn(move || {
        let stream_handle =
            rodio::OutputStreamBuilder::open_default_stream().expect("Cant open stream");
        let sink = Sink::connect_new(stream_handle.mixer());

        let mut song_playing: bool = false;
        let mut songs: Vec<String> = Vec::new();
        loop {
            if sink.empty() && song_playing && !songs.is_empty() {
                sink.append(create_song(songs[0].to_string()));
                songs.remove(0);
            };
            //Music player channel recieving
            //
            let recieved_mp = rx_mp.try_recv().ok();
            if let Some(message) = recieved_mp {
                match message {
                    MusicPlayer::Pause => {
                        if sink.is_paused() {
                            sink.play()
                        } else {
                            sink.pause()
                        }
                        song_playing = false
                    }
                    MusicPlayer::Stop => {
                        sink.clear();
                        song_playing = false
                    }
                    MusicPlayer::Volume(volume) => {
                        //This seems really stupid but it works
                        if volume == 0.0 {
                            sink.set_volume(0.0);
                        } else {
                            sink.set_volume(volume);
                        }
                    }
                    MusicPlayer::NewSong(song) => {
                        sink.clear();
                        let song = create_song(song);
                        sink.append(song);
                        sink.play();
                        song_playing = true;
                    }
                    MusicPlayer::End => {
                        break;
                    } //Queue handling
                }
            }

            let recieved_q = rx_q.try_recv().ok();
            if let Some(message) = recieved_q {
                match message {
                    Queue::Add(song) => {
                        if sink.empty() {
                            let song = create_song(song);
                            sink.append(song);
                            sink.play();
                        } else {
                            songs.push(song);
                        }
                    }
                    Queue::Remove(index) => {
                        songs.remove(index - 1);
                    }
                    Queue::Clear => {
                        songs.clear();
                    }
                }
            }
            is_song_playing(&sink, &songs);
        }
    });

    (tx_mp.clone(), tx_q.clone())
}

fn create_song(song: String) -> Decoder<BufReader<File>> {
    let file =
        BufReader::new(File::open(&song).unwrap_or_else(|_| panic!("Cant read file: {song}")));
    Decoder::new(file).unwrap()
}

fn is_song_playing(sink: &Sink, songs: &[String]) -> bool {
    if sink.empty() {
        if let Some(song) = songs.first() {
            let song = create_song(song.clone());
            sink.append(song);
        }
        return false;
    }
    true
}
