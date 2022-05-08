mod infer;
mod infer_single;
mod lofty_validation;
mod mime_check_extensions;

use crate::infer::infer_check;
use crate::lofty_validation::lofty_check;
use crate::mime_check_extensions::mime_check;

use crate::infer_single::infer_single;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use walkdir::WalkDir;

fn main() {
    // lofty_check();
    // mime_check();
    // infer_check();
    infer_single();
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
                let extension = extension.to_string_lossy().to_string();
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
    println!("Collected files to scan");
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
