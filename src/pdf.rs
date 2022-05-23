use crate::collect_files;
use pdf::object::ParseOptions;
use pdf::PdfError;
use pdf::PdfError::{InvalidPassword, Try};
use rayon::prelude::*;
use std::{fs, panic};

pub fn pdf_check(directories: Vec<&str>) {
    let allowed_extensions = vec!["pdf"];
    let disabled_extensions = vec![];

    let collected_files = collect_files(directories, allowed_extensions, disabled_extensions);

    collected_files
        .into_par_iter()
        .for_each(|(path, _extension)| {
            // println!("Checking {}", path);

            let content = match fs::read(&path) {
                Ok(t) => t,
                Err(_) => return,
            };

            let result = panic::catch_unwind(|| {
                let parser_options = ParseOptions {
                    allow_error_in_option: true,
                    allow_xref_error: true,
                    allow_invalid_ops: true,
                    allow_missing_endobj: true,
                };
                if let Err(e) = pdf::file::File::from_data_with_options(content, parser_options) {
                    let error = unpack_pdf_error(e);
                    if let InvalidPassword = error {
                        return;
                    }
                    println!("{}    -     {}", path, error);
                } else {
                    // println!("VALID   {}", path);
                }
            });

            if let Err(_e) = result {
                println!(
                    "BIG ERROR - {} crashed please report bug to https://github.com/pdf-rs/pdf/",
                    path
                );
            }
        });
}

fn unpack_pdf_error(e: PdfError) -> PdfError {
    if let Try {
        file: _,
        line: _,
        column: _,
        context: _,
        source,
    } = e
    {
        unpack_pdf_error(*source)
    } else {
        e
    }
}
