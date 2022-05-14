use crate::collect_files;
use rayon::prelude::*;
use std::path::Path;

pub fn image_check(directories: Vec<&str>) {
    let allowed_extensions = vec![
        "jpg", "jpeg", "png", "bmp", "tiff", "tif", "tga", "ff", "jif", "jfi", "webp", "gif",
        "ico", "exr", "hdr",
    ];
    let disabled_extensions = vec![];

    let collected_files = collect_files(directories, allowed_extensions, disabled_extensions);

    collected_files
        .into_par_iter()
        .for_each(|(path, _extension)| {
            if let Err(e) = image::open(Path::new(&path)) {
                println!("{}       -       {}", path, e);
            } else {
                // println!("{}", path); // Enable in case of trying to find image extensions that are suported by image-rs library
            }
        });
}
