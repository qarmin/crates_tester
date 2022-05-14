use crate::{collect_files, count_number_of_vec_items};
use std::path::Path;

pub fn mime_check(directories: Vec<&str>, print_results: bool) {
    let allowed_extensions = vec![];
    let excluded_extensions = vec![];

    let collected_files = collect_files(directories, allowed_extensions, excluded_extensions);

    let collected_extensions: Vec<_> = collected_files
        .into_iter()
        .filter_map(|path| {
            let path = Path::new(&path);
            if let Some(extension) = path.extension() {
                if extension.len() > 10 {
                    return None; // This is mostly a really invalid extension
                }
                let extension = extension.to_string_lossy().to_string();
                let mime_g = mime_guess::from_ext(extension.as_str());
                let mime_number = mime_g.iter().count();
                if mime_number == 0 {
                    if print_results {
                        println!("{:?}", path);
                    }
                    return Some(extension);
                }
            }
            None
        })
        .collect();
    let new_vec = count_number_of_vec_items(collected_extensions);

    for (ext, number) in new_vec {
        println!("{} - {}", number, ext);
    }
}
