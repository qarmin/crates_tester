use crate::{collect_files, count_number_of_vec_items};
use rayon::prelude::*;
use std::path::Path;
use std::process::Command;

pub fn infer_check(directories: Vec<&str>, print_results: bool) {
    let allowed_extensions = vec![];
    let disabled_extensions = vec!["cache", "file"];

    let collected_files = collect_files(directories, allowed_extensions, disabled_extensions);

    let extensions: Vec<_> = collected_files
        .into_par_iter()
        .filter_map(|path| {
            if let Some(extension) = Path::new(&path).extension() {
                let extension = extension.to_string_lossy().to_lowercase();
                if extension.len() > 10 {
                    return None; // This is mostly a really invalid extension
                }
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
