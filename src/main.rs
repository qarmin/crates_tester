use lofty::{AudioFile, ItemKey, read_from_path};
use walkdir::WalkDir;

fn main() {
    let allowed_extensions: &[&str] = &["mp3","flac","ogg","wav"];
    let checked_dir = "/mnt/";
    println!("Hello, world!");

    for entry in WalkDir::new(checked_dir).into_iter().filter_map(|e| e.ok()) {

        let path = entry.path();
        if let Some(extension) = path.extension(){
            let extension = extension.to_string_lossy().to_string();
            if !allowed_extensions.contains(&extension.as_str()){
                continue;
            }
        }
        else{
            continue;
        }

        if ["/home/rafal/.cargo/registry/src/github.com-1ecc6299db9ec823/claxon-0.4.3/testsamples/short.flac","/home/rafal/test/Untitled Folder 2/sample (1 307th copy)97.m4a"].contains(&path.to_str().unwrap()){
            continue;
        }

        // println!("Checking {:?}", path);

        let tagged_file = match read_from_path(&path, true) {
            Ok(t) => t,
            Err(e) => {
                println!("Invalid file - {}, {}", path.to_string_lossy(),e);
               continue;
            }
        };

        let properties = tagged_file.properties();

        let mut track_title : String = "".to_string();
        let mut track_artist: String= "".to_string();
        let mut year: String= "".to_string();
        let mut genre: String= "".to_string();

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
        if let Ok(mut length_number) = length.parse::<u32>() {
            length_number /= 60;
            let minutes = length_number / 1000;
            let seconds = (length_number % 1000) * 6 / 100;
            if minutes != 0 || seconds != 0 {
                length = format!("{}:{:02}", minutes, seconds);
            } else if length_number > 0 {
                // That means, that audio have length smaller that second, but length is properly read
                length = "0:01".to_string();
            } else {
                length = "".to_string();
            }
        } else {
            length = "".to_string();
        }

        if length.is_empty() || bitrate == 0 {
            println!("{} - length {} - length_old {} - bitrate - {}", entry.path().display(), length, old_number, bitrate);
        }
    }

}
