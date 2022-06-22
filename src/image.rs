use crate::collect_files;
use rayon::prelude::*;
use std::panic;
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

            let result = panic::catch_unwind(|| {
                if let Err(e) = image::open(Path::new(&path)) {
                    println!("{}       -       {}", path, e);
                } else {
                    // println!("{}", path); // Enable in case of trying to find image extensions that are suported by image-rs library
                }

                // Alternative mode with guessing formats
                // let mut file = match File::open(&path){
                //     Ok(t) => t,
                //     Err(_) => return
                // };
                // let mut content = Vec::new();
                // file.read_to_end(&mut content);
                // if let Err(e) = image::load_from_memory(&content) {
                //     println!("{}       -       {}", path, e);
                // } else {
                //     // println!("{}", path); // Enable in case of trying to find image extensions that are supported by image-rs library
                // }
            });

            if let Err(_e) = result {
                println!(
                    "BIG ERROR - {} crashed please report bug to https://github.com/image-rs/image/",
                    path
                );
            }
        });
}
