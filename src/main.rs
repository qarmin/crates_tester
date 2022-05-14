mod image;
mod infer;
mod infer_single;
mod lofty_validation;
mod mime_check_extensions;
mod symphonia;
mod zip;

use crate::infer::infer_check;
use crate::lofty_validation::lofty_check;
use crate::mime_check_extensions::mime_check;

use crate::image::image_check;

use crate::symphonia::symphonia_check;
use crate::zip::zip_check;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::hash::Hash;
use walkdir::WalkDir;

/// Excluded extensions
fn main() {
    let included_directories: Vec<_> = env::args().collect();
    if included_directories.len() < 3 {
        println!("App must provide at least type of check and directories");
        return;
    }
    let type_check = &included_directories[1];
    let mut print_results = false;
    let mut directories_to_check = Vec::new();
    let mut included_extensions = Vec::new();
    let mut excluded_extensions = Vec::new();
    for thing in included_directories.iter().skip(2) {
        if let Some(good) = thing.strip_prefix("-i") {
            included_extensions.push(good);
        } else if let Some(good) = thing.strip_prefix("-e") {
            excluded_extensions.push(good);
        } else if thing == "PRINT" {
            print_results = true;
        } else {
            directories_to_check.push(thing.as_str());
        }
    }

    match type_check.to_ascii_lowercase().as_str() {
        "image" => {
            image_check(directories_to_check);
        }
        "lofty" => {
            lofty_check(directories_to_check);
        }
        "mime" => {
            mime_check(directories_to_check, print_results);
        }
        "infer" => {
            infer_check(directories_to_check, print_results);
        }
        "zip" => {
            zip_check(directories_to_check);
        }
        "symphonia" => {
            symphonia_check(directories_to_check);
        }
        e => {
            println!("Not supported option - {}", e)
        }
    }
}

fn collect_files(
    checked_dirs: Vec<&str>,
    allowed_extensions: Vec<&str>,
    excluded_extensions: Vec<&str>,
) -> Vec<String> {
    let mut collected_files = Vec::new();
    for dir in checked_dirs {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if path.to_string_lossy().contains(".cargo") {
                continue;
            }

            if let Some(extension) = path.extension() {
                let extension = extension.to_string_lossy().to_lowercase();
                if !allowed_extensions.is_empty()
                    && !allowed_extensions.contains(&extension.as_str())
                {
                    continue;
                }
                if excluded_extensions.contains(&extension.as_str()) {
                    continue;
                }
            } else {
                continue;
            }

            collected_files.push(path.to_string_lossy().to_string());
        }
    }
    println!("Collected files to scan({})", collected_files.len());
    collected_files
}

fn count_number_of_vec_items<T>(vec: Vec<T>) -> Vec<(T, u32)>
where
    T: Eq + Hash + Clone,
{
    let mut btree: HashMap<T, u32> = Default::default();
    for i in vec {
        btree.entry(i.clone()).or_insert(0);
        *btree.get_mut(&i).unwrap() += 1;
    }

    let mut new_vec: Vec<(T, u32)> = Vec::new();
    for i in btree {
        new_vec.push(i);
    }
    new_vec.sort_by(|(_, number), (_, number2)| {
        if number < number2 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });
    new_vec
}
