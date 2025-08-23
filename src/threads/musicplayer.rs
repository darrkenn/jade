use crossbeam_channel::{Receiver, Sender, bounded, unbounded};
use rodio::{Decoder, Sink};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use std::{panic, thread};

use crate::Info;

pub enum MusicPlayer {
    Pause,
    Stop,
    Volume(f32),
    NewSong(String, u32),
}

pub struct Request;

pub fn create_mp(
    volume: f32,
    s_info: Sender<Info>,
) -> (
    Sender<MusicPlayer>,
    Receiver<MusicPlayer>,
    Receiver<Request>,
) {
    //Channels
    let (s_mp, r_mp) = unbounded::<MusicPlayer>();
    let (s_mp_clone, r_mp_clone) = (s_mp.clone(), r_mp.clone());

    let (s_req, r_req) = bounded::<Request>(1);
    let r_req_clone = r_req.clone();
    thread::spawn(move || {
        //Sink setup
        let stream_handle =
            rodio::OutputStreamBuilder::open_default_stream().expect("Cant open stream");
        let sink = Sink::connect_new(stream_handle.mixer());
        sink.set_volume(volume);

        loop {
            if sink.empty() {
                s_info
                    .send(Info::Clear)
                    .expect("Couldnt send message to info thread");

                s_req.send(Request).expect("Cant send request");
                thread::sleep(Duration::from_millis(30));
            } else {
                s_info
                    .send(Info::Position(sink.get_pos().as_secs() as u32))
                    .expect("Couldnt send message to info thread");
            };

            let recieved_mp = r_mp.recv_timeout(Duration::from_millis(10)).ok();
            if let Some(message) = recieved_mp {
                match message {
                    MusicPlayer::Pause => {
                        if sink.is_paused() {
                            sink.play()
                        } else {
                            sink.pause()
                        }
                    }
                    MusicPlayer::Stop => {
                        s_info
                            .send(Info::Clear)
                            .expect("Couldnt send message to info thread");
                        sink.clear();
                    }
                    MusicPlayer::Volume(volume) => {
                        //This seems really stupid but it works
                        if volume == 0.0 {
                            sink.set_volume(0.0);
                        } else {
                            sink.set_volume(volume);
                        }
                    }
                    MusicPlayer::NewSong(song, length) => {
                        sink.clear();
                        s_info
                            .send(Info::Song(song.clone(), length))
                            .expect("Couldnt send message to info thread");
                        let song = create_song(song);
                        sink.append(song);
                        sink.play();
                    }
                }
            }
        }
    });
    (s_mp_clone, r_mp_clone, r_req_clone)
}

fn create_song(song: String) -> Decoder<BufReader<File>> {
    let file =
        BufReader::new(File::open(&song).unwrap_or_else(|_| panic!("Cant read file: {song}")));
    Decoder::new(file).unwrap()
}
