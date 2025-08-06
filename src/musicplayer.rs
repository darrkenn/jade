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
    End,
}

pub fn create_mp(volume: f32) -> (Sender<MusicPlayer>, Sender<Queue>) {
    let (tx_mp, rx_mp) = mpsc::channel::<MusicPlayer>();
    let (tx_q, rx_q) = mpsc::channel::<Queue>();
    thread::spawn(move || {
        let stream_handle =
            rodio::OutputStreamBuilder::open_default_stream().expect("Cant open stream");
        let sink = Sink::connect_new(stream_handle.mixer());
        sink.set_volume(volume);

        let mut song_playing: bool = false;
        let mut songs: Vec<String> = Vec::new();
        loop {
            if sink.empty() && song_playing && !songs.is_empty() {
                sink.append(create_song(songs[0].to_string()));
                songs.remove(0);
            };

            //Music player channel recieving
            let received_mp = match rx_mp.try_recv() {
                Ok(message) => message,
                Err(mpsc::TryRecvError::Empty) => {
                    continue;
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    break;
                }
            };
            match received_mp {
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
                        let current_pos = sink.get_pos().as_secs_f64().round() as u64;
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

            let recieved_q = match rx_q.try_recv() {
                Ok(message) => message,
                Err(mpsc::TryRecvError::Empty) => {
                    continue;
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    break;
                }
            };

            match recieved_q {
                Queue::Add(song) => {
                    songs.push(song);
                }
                Queue::Remove(index) => {
                    songs.remove(index);
                }
                Queue::Clear => {
                    songs.clear();
                }
                Queue::End => {
                    break;
                }
            }
        }
    });

    (tx_mp.clone(), tx_q.clone())
}

fn create_song(song: String) -> Decoder<BufReader<File>> {
    let file =
        BufReader::new(File::open(&song).unwrap_or_else(|_| panic!("Cant read file: {song}")));
    Decoder::new(file).unwrap()
}
