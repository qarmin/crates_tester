use crate::collect_files;
use rayon::prelude::*;
use std::fs::File;
use std::io;
use std::panic;

use symphonia::core::codecs::CODEC_TYPE_NULL;
use symphonia::core::errors::Error;
use symphonia::core::errors::Error::IoError;
use symphonia::core::io::MediaSourceStream;

pub fn symphonia_check(directories: Vec<&str>) {
    let allowed_extensions = vec![
        "mp3", "flac", "wav", "ogg", "m4a", "aac", "aiff", "pcm", "ac3", "aif", "aiff", "aifc",
        "m3a", "mp2", "mp4a", "mp2a", "mpga", "opus", "wave", "weba", "wma", "oga",
    ];
    let disabled_extensions = vec![];

    let collected_files = collect_files(directories, allowed_extensions, disabled_extensions);

    collected_files.into_par_iter().for_each(|(path, _extension)| {
        if let Ok(file) = File::open(&path) {
            // println!("Checking {}", path);

            let result = panic::catch_unwind(|| {
                if let Err(e) = parse_audio_file(file) {
                    println!("{}    -     {}", path, e);
                } else {
                    // println!("VALID   {}", path);
                }
            });

            if let Err(_e) = result {
                println!("BIG ERROR - {} crashed please report bug to https://github.com/pdeljanov/Symphonia", path);
            }
        }
    });
}

pub fn parse_audio_file(file_handler: File) -> Result<(), Error> {
    let mss = MediaSourceStream::new(Box::new(file_handler), Default::default());

    let probed = match symphonia::default::get_probe().format(
        &Default::default(),
        mss,
        &Default::default(),
        &Default::default(),
    ) {
        Ok(t) => t,
        Err(_) => {
            return Err(Error::Unsupported(
                "probe info not available/file not recognized",
            ))
        }
    };

    let mut format = probed.format;

    let track = match format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
    {
        Some(k) => k,
        None => return Err(Error::Unsupported("not supported audio track")),
    };

    let mut decoder =
        match symphonia::default::get_codecs().make(&track.codec_params, &Default::default()) {
            Ok(k) => k,
            Err(_) => return Err(Error::Unsupported("not supported codec")),
        };

    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(Error::ResetRequired) => {
                return Err(Error::ResetRequired);
            }
            Err(err) => {
                if let IoError(ref er) = err {
                    // Catch eof, not sure how to do it properly
                    if er.kind() == io::ErrorKind::UnexpectedEof {
                        return Ok(());
                    }
                }
                return Err(err);
            }
        };

        decoder.decode(&packet)?;
    }
}
