use std::io::{Cursor, Write};
use std::path::Path;
use std::fs::File;

use image::imageops::FilterType;
use image::ImageFormat;
use rayon::prelude::*;
use zip::write::SimpleFileOptions;
use zip::CompressionMethod;
use zip::ZipWriter;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let input = match args.iter().position(|a| a == "--input").and_then(|i| args.get(i + 1)) {
        Some(v) => v.clone(),
        None => {
            eprintln!("Usage: chhobi --input <folder> --output <zip-file>");
            std::process::exit(1);
        }
    };

    let output = match args.iter().position(|a| a == "--output").and_then(|i| args.get(i + 1)) {
        Some(v) => v.clone(),
        None => {
            eprintln!("Usage: chhobi --input <folder> --output <zip-file>");
            std::process::exit(1);
        }
    };

    let input_path = Path::new(&input);
    if !input_path.is_dir() {
        eprintln!("Error: input path '{}' is not a directory", input);
        std::process::exit(1);
    }

    let paths: Vec<_> = ignore::WalkBuilder::new(input_path)
        .standard_filters(false)
        .build()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if !entry.file_type()?.is_file() {
                return None;
            }
            let ext = path.extension()?.to_str()?.to_lowercase();
            if matches!(ext.as_str(), "jpg" | "jpeg" | "png") {
                Some(path.to_path_buf())
            } else {
                None
            }
        })
        .collect();

    if paths.is_empty() {
        eprintln!("No images found in {}", input);
        std::process::exit(1);
    }

    eprintln!("Found {} images. Processing...", paths.len());

    let results: Vec<(String, Vec<u8>, Vec<u8>)> = paths
        .par_iter()
        .filter_map(|path| {
            let name = path.file_name()?.to_str()?.to_string();
            let format = ImageFormat::from_extension(path.extension()?)?;

            let img = image::open(path).ok()?;

            let passport = img.resize_exact(600, 600, FilterType::Lanczos3);
            let stamp = img.resize_exact(300, 300, FilterType::Lanczos3);

            let mut passport_buf = Cursor::new(Vec::new());
            passport.write_to(&mut passport_buf, format).ok()?;

            let mut stamp_buf = Cursor::new(Vec::new());
            stamp.write_to(&mut stamp_buf, format).ok()?;

            eprintln!("  Processed: {}", name);

            Some((name, passport_buf.into_inner(), stamp_buf.into_inner()))
        })
        .collect();

    let file = File::create(&output).expect("Cannot create output file");
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);

    for (name, passport, stamp) in &results {
        zip.start_file(format!("passport/{}", name), options)
            .expect("Cannot write passport entry to zip");
        zip.write_all(passport).expect("Cannot write passport data to zip");

        zip.start_file(format!("stamp/{}", name), options)
            .expect("Cannot write stamp entry to zip");
        zip.write_all(stamp).expect("Cannot write stamp data to zip");
    }

    zip.finish().expect("Cannot finalize zip");

    eprintln!("Done! Archive created: {}", output);
}