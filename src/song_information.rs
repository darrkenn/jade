use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use symphonia::core::io::MediaSourceStream;
use symphonia::default::get_probe;
use crate::SUPPORTED_FORMATS;

pub fn get_songs_in_folder(music_folder: PathBuf) -> (Vec<String>, Vec<u32>, Vec<String>) {
    let mut songs = Vec::new();
    let mut files: Vec<File> = Vec::new();
    if let Ok(entries) = fs::read_dir(music_folder) {
        for entry in entries {
            if let Ok(entry) = entry {
                if SUPPORTED_FORMATS.contains(&entry.path().extension().unwrap().to_str().unwrap()) {
                    songs.push(entry.file_name().to_str().unwrap().to_string());
                    let file = &entry.path().to_string_lossy().to_string();
                    let path = Path::new(&file);
                    files.push(File::open(&path).expect("Cant open file"))
                }
            }
        }
    }

    let song_lengths = get_song_lengths(files);
    let visual_song_lengths = get_visual_lengths(song_lengths.clone());
    (songs, song_lengths, visual_song_lengths)
}

fn get_song_lengths(files: Vec<File>) -> Vec<u32> {
    let mut lengths: Vec<u32> = Vec::new();
    for file in  files {
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let probe = get_probe()
            .format(&Default::default(), mss, &Default::default(), &Default::default())
            .expect("Cant probe file");

        let format = probe.format;
        let track = format.default_track().expect("Cant find track");
        if let Some(sample_rate) = track.codec_params.sample_rate {
            if let Some(frames) = track.codec_params.n_frames {
                let length = frames as f64 / sample_rate as f64;
                lengths.push(length.round() as u32)
            }
        }
    }
    lengths
}

fn get_visual_lengths(lengths: Vec<u32>) -> Vec<String> {
    let mut visual_lengths: Vec<String> = Vec::new();
    for length in lengths.into_iter() {
        let mins = length / 60;
        let secs = length % 60;
        if secs < 10 {
            visual_lengths.push(format!("{}:0{}", mins,secs))
        } else {
            visual_lengths.push(format!("{}:{}", mins,secs))
        }
    }
    visual_lengths
}