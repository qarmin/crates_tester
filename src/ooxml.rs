// use crate::collect_files;
// use ooxml::document::SpreadsheetDocument;
// use rayon::prelude::*;
// use std::panic;
//
// pub fn ooxml_check(directories: Vec<&str>) {
//     let allowed_extensions = vec!["xlsx"];
//     let disabled_extensions = vec![];
//
//     let collected_files = collect_files(directories, allowed_extensions, disabled_extensions);
//
//     collected_files.into_par_iter().for_each(|(path,_extension)| {
//             // println!("Checking {}", path);
//
//             let result = panic::catch_unwind(|| {
//                 if let Err(e) = SpreadsheetDocument::open(&path) {
//                     println!("{}    -     {:?}", path, e);
//                 } else {
//                     // println!("VALID   {}", path);
//                 }
//             });
//
//             if let Err(_e) = result {
//                 println!("BIG ERROR - {} crashed please report bug to https://github.com/zitsen/ooxml-rs/", path);
//             }
//     });
// }
