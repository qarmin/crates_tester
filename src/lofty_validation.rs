use crate::collect_files;
use lofty::error::ErrorKind::Io;
use lofty::{read_from, AudioFile, ItemKey};
use rayon::prelude::*;
use std::fs::File;

pub fn lofty_check(directories: Vec<&str>) {
    let allowed_extensions = vec![
        "mp3", "flac", "wav", "m4a", "aac", "aiff", "pcm", "ac3", "aif", "aiff", "aifc", "m3a",
        "mp2", "mp4a", "mp2a", "mpga", "opus", "wave", "weba", "wma", "oga", "ogg",
    ];
    let excluded_extensions = vec![
        "mp4", "3gp", // Not interested in video files, but looks that are supported
    ];

    let collected_files = collect_files(directories, allowed_extensions, excluded_extensions);

    collected_files
        .into_par_iter()
        .for_each(|(path, _extension)| {
            let mut file = match File::open(&path) {
                Ok(t) => t,
                Err(_) => return,
            };
            let tagged_file = match read_from(&mut file, true) {
                Ok(t) => t,
                Err(e) => {
                    println!("Invalid file - {}, {}", path, e);

                    match e.kind() {
                        Io(_) => {}
                        _ => {
                            println!("Invalid file - {}, {}", path, e)
                        }
                    }
                    return;
                }
            };
            // println!("Valid file - {}", path);
            let properties = tagged_file.properties();

            let mut track_title: String = "".to_string();
            let mut track_artist: String = "".to_string();
            let mut year: String = "".to_string();
            let mut genre: String = "".to_string();

            let bitrate = properties.audio_bitrate().unwrap_or(0);
            let mut length = properties.duration().as_millis().to_string();

            for tag in tagged_file.tags() {
                if track_title.is_empty() {
                    if let Some(tag_value) = tag.get_string(&ItemKey::TrackTitle) {
                        track_title = tag_value.to_string();
                    }
                }
                if track_artist.is_empty() {
                    if let Some(tag_value) = tag.get_string(&ItemKey::TrackArtist) {
                        track_artist = tag_value.to_string();
                    }
                }
                if year.is_empty() {
                    if let Some(tag_value) = tag.get_string(&ItemKey::Year) {
                        year = tag_value.to_string();
                    }
                }
                if genre.is_empty() {
                    if let Some(tag_value) = tag.get_string(&ItemKey::Genre) {
                        genre = tag_value.to_string();
                    }
                }
                // println!("{:?}", tag.items());
            }

            // println!("{:?}", tag.items());
            let old_number = length.clone();
            if let Ok(old_length_number) = length.parse::<u32>() {
                let length_number = old_length_number / 60;
                let minutes = length_number / 1000;
                let seconds = (length_number % 1000) * 6 / 100;
                if minutes != 0 || seconds != 0 {
                    length = format!("{}:{:02}", minutes, seconds);
                } else if old_length_number > 0 {
                    // That means, that audio have length smaller that second, but length is properly read
                    length = "0:01".to_string();
                } else {
                    length = "".to_string();
                }
            } else {
                length = "".to_string();
            }

            if length.is_empty() || bitrate == 0 {
                println!(
                    "{} - length {} - length_old {} - bitrate - {}",
                    path, length, old_number, bitrate
                );
            }
        });
}
