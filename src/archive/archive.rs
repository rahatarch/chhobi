use std::fs::File;
use std::io::Write;

use zip::write::SimpleFileOptions;
use zip::CompressionMethod;
use zip::ZipWriter;

pub fn create_zip(output: &str, entries: &[(String, &[u8], &[u8])]) {
    let file = match File::create(output) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Cannot write to output path. Check permissions: {}", e);
            std::process::exit(1);
        }
    };

    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);

    for (name, passport, stamp) in entries {
        if let Err(e) = zip.start_file(format!("passport/{}", name), options) {
            eprintln!("Error: cannot write passport entry for '{}': {}", name, e);
            std::process::exit(1);
        }
        if let Err(e) = zip.write_all(passport) {
            eprintln!("Error: cannot write passport data for '{}': {}", name, e);
            std::process::exit(1);
        }

        if let Err(e) = zip.start_file(format!("stamp/{}", name), options) {
            eprintln!("Error: cannot write stamp entry for '{}': {}", name, e);
            std::process::exit(1);
        }
        if let Err(e) = zip.write_all(stamp) {
            eprintln!("Error: cannot write stamp data for '{}': {}", name, e);
            std::process::exit(1);
        }
    }

    if let Err(e) = zip.finish() {
        eprintln!("Error: cannot finalize zip archive: {}", e);
        std::process::exit(1);
    }
}