use crate::{collect_files, count_number_of_vec_items};
use rayon::prelude::*;
use std::path::Path;
use std::process::Command;

pub fn infer_check(directories: Vec<&str>, print_results: bool) {
    let allowed_extensions = vec![
        // This extensions should be supported by inder
        "cache", "file", "data", // // This extensions should be supported by inder
        "jpg", "png", "gif", "webp", "cr2", "tif", "tiff", "bmp", "avif", "jxr", "psd", "ico",
        "mp4", "m4v", "mkv", "webm", "mov", "avi", "vmv", "mpg", "flv", "mid", "mp3", "m4a", "ogg",
        "flac", "wav", "amr", "aac", "aiff", "dsf", "ape", "epub", "zip", "tar", "rar", "gz",
        "bz2", "7z", "xz", "pdf", "swf", "rtf", "eot", "ps", "sqlite", "nes", "crx", "cab", "deb",
        "ar", "la", "rpm", "dcm", "zst", "msi", "mobi", "doc", "docx", "xls", "xlsx", "ppt",
        "pptx", "odt", "ods", "odp", "woff", "woff2", "ttf", "otf", "wasm", "exe", "dl", "elf",
        "bc", "mach", "class", "dex", "dey", "der",
    ];
    let disabled_extensions = vec![
        // Obj is used in a lot different ways
        "obj",
    ];

    let collected_files = collect_files(directories, allowed_extensions, disabled_extensions);

    let extensions: Vec<_> = collected_files
        .into_par_iter()
        .filter_map(|(path, extension)| {
            match infer::get_from_path(Path::new(&path)) {
                Ok(good) => {
                    if good.is_none() {
                        // Prints also
                        if print_results {
                            let output = match Command::new("file").arg(&path).output() {
                                Ok(t) => t,
                                Err(e) => {
                                    println!("Failed to run file command, reason {}", e);
                                    return None;
                                }
                            };
                            let mut output = String::from_utf8(output.stdout).unwrap();
                            output.pop(); // Removes new line \n
                            println!("{} - {}", path, output);
                        }
                        return Some(extension);
                    }
                }
                Err(e) => {
                    println!("{} - {}", path, e);
                }
            }

            None
        })
        .collect();
    let new_vec = count_number_of_vec_items(extensions);
    for (ext, number) in new_vec {
        if number > 0 {
            println!("{} - {}", number, ext);
        }
    }
}
