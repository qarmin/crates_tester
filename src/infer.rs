use crate::{collect_files, count_number_of_vec_items};
use rayon::prelude::*;
use std::path::Path;
use std::process::Command;
use std::sync::atomic::compiler_fence;

pub fn infer_check() {
    let allowed_extensions = vec![];
    let disabled_extensions = vec!["cache", "file"];
    let checked_dir = vec!["/home/"]; //,"/mnt/","/media/rafal/Disk/Untitled Folder"];

    let collected_files = collect_files(checked_dir, allowed_extensions, disabled_extensions);

    let extensions_to_print = ["xml"];
    let extensions: Vec<_> = collected_files
        .into_par_iter()
        .filter_map(|path| {
            match infer::get_from_path(Path::new(&path)) {
                Ok(good) => {
                    if good.is_none() {
                        if let Some(extension) = Path::new(&path).extension() {
                            let extension = extension.to_string_lossy().to_lowercase();
                            if extensions_to_print.contains(&extension.as_str()) {
                                let output = Command::new("file")
                                    .arg(&path)
                                    .output()
                                    .expect("Failed to execute command");
                                let mut output = String::from_utf8(output.stdout).unwrap();
                                output.pop(); // Removes new line \n
                                println!("{} - {}", path, output);
                            }
                            return Some(extension);
                        }
                    }
                }
                Err(e) => {
                    println!("WRONG SOMEThING {} - {}", e, path);
                }
            }

            None
        })
        .collect();
    let new_vec = count_number_of_vec_items(extensions);
    for (ext, number) in new_vec {
        if number > 100 {
            println!("{} - {}", number, ext);
        }
    }
}
