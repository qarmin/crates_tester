use crate::collect_files;
use rayon::prelude::*;
use std::fs;

pub fn zip_check(directories: Vec<&str>) {
    let allowed_extensions = vec!["zip"];
    let disabled_extensions = vec![
        // This are zip files, but don't want to check them if are valid
        "doc", "kra", "jar", "ods", "pptx", "xpi", "odg", "odt", "docx", "oxt", "odp", "apk",
        "zcos", "xlsx", "ppsx", "ppt", "xls", "epub",
    ];

    let collected_files = collect_files(directories, allowed_extensions, disabled_extensions);

    collected_files.into_par_iter().for_each(|path| {
        if let Ok(file) = fs::File::open(&path) {
            if let Err(e) = zip::ZipArchive::new(file) {
                println!("{}    -     {}", path, e);
            } else {
                // println!("VALID   {}", path);
            }
        }
    });
}
