use lofty::error::ErrorKind::{Io, UnknownFormat};
use lofty::{read_from_path, AudioFile, ItemKey};
use rayon::prelude::*;
use walkdir::WalkDir;

fn main() {
    lofty_check();
    // mime_type_found_missing()
}

fn collect_files(checked_dirs: Vec<&str>, allowed_extensions: Vec<&str>) -> Vec<String> {
    let mut collected_files = Vec::new();
    for dir in checked_dirs {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.to_string_lossy().contains(".cargo") {
                continue;
            }
            if let Some(extension) = path.extension() {
                let extension = extension.to_string_lossy().to_string();
                if !allowed_extensions.contains(&extension.as_str()) {
                    continue;
                }
            } else {
                continue;
            }
            collected_files.push(path.to_string_lossy().to_string());
        }
    }
    println!("Collected files to scan");
    collected_files
}

fn lofty_check() {
    let allowed_extensions = vec![
        "mp3", "flac", "wav", "ogg", "m4a", "aac", "aiff", "pcm", "ac3", "aif", "aiff", "aifc",
        "m3a", "mp2", "mp4a", "mp2a", "mpga", "oga", "opus", "wave", "weba", "wma",
    ];
    let checked_dir = vec!["/home/"]; //,"/mnt/","/media/rafal/Disk/Untitled Folder"];

    let collected_files = collect_files(checked_dir, allowed_extensions);

    collected_files.into_par_iter().for_each(|path| {
        if path.contains(".cargo") {
            return;
        }
        let tagged_file = match read_from_path(&path, true) {
            Ok(t) => t,
            Err(e) => {
                match e.kind() {
                    UnknownFormat | Io(_) => {}
                    _ => {
                        println!("Invalid file - {}, {}", path, e)
                    }
                }
                println!("Invalid file - {}, {}", path, e);
                return;
            }
        };

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
