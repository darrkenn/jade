use crossbeam_channel::{Receiver, Sender, bounded, unbounded};
use rodio::{Decoder, Sink};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use std::{panic, thread};

pub enum MusicPlayer {
    Pause,
    Stop,
    Volume(f32),
    NewSong(String),
    End,
}

pub struct Request;

pub fn create_mp(
    volume: f32,
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
                s_req.send(Request);
                thread::sleep(Duration::from_millis(30));
            }

            let recieved_mp = r_mp.try_recv().ok();
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
                    MusicPlayer::NewSong(song) => {
                        sink.clear();
                        let song = create_song(song);
                        sink.append(song);
                        sink.play();
                    }
                    MusicPlayer::End => {
                        break;
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
