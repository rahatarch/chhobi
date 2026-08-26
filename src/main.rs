use std::collections::HashSet;
use std::io::{Cursor, Write};
use std::path::Path;
use std::fs::File;

use image::GenericImageView;
use image::DynamicImage;
use image::imageops::FilterType;
use image::ImageFormat;
use rayon::prelude::*;
use zip::write::SimpleFileOptions;
use zip::CompressionMethod;
use zip::ZipWriter;

fn crop_to_square(img: &DynamicImage) -> DynamicImage {
    let (w, h) = img.dimensions();
    let size = w.min(h);
    let x = (w - size) / 2;
    let y = (h - size) / 2;
    img.crop_imm(x, y, size, size)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let help = r"chhobi — batch crop and resize images for passport/stamp printing

Usage:
  chhobi --input <folder> --output <zip-file>

Flags:
  --input   <folder>     Directory containing JPG/PNG images
  --output  <zip-file>   Path for the output ZIP archive
  --help                 Show this help message and exit
  --version              Show version information and exit
";

    if args.iter().any(|a| a == "--help" || a == "-h") {
        print!("{}", help);
        return;
    }

    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("chhobi {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let input = match args.iter().position(|a| a == "--input").and_then(|i| args.get(i + 1)) {
        Some(v) => v.clone(),
        None => {
            eprint!("{}", help);
            std::process::exit(1);
        }
    };

    let output = match args.iter().position(|a| a == "--output").and_then(|i| args.get(i + 1)) {
        Some(v) => v.clone(),
        None => {
            eprint!("{}", help);
            std::process::exit(1);
        }
    };

    let input_path = Path::new(&input);
    if !input_path.is_dir() {
        eprintln!("Error: input path '{}' is not a directory", input);
        std::process::exit(1);
    }

    let output_path = Path::new(&output);
    if let Some(parent) = output_path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            eprintln!("Cannot write to output path. Check permissions.");
            std::process::exit(1);
        }
        if parent.exists() && !parent.is_dir() {
            eprintln!("Cannot write to output path. Check permissions.");
            std::process::exit(1);
        }
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

    struct FileResult {
        name: String,
        passport: Vec<u8>,
        stamp: Vec<u8>,
        skip_reason: Option<String>,
    }

    let results: Vec<FileResult> = paths
        .par_iter()
        .map(|path| {
            let name = match path.file_name().and_then(|s| s.to_str()) {
                Some(n) => n.to_string(),
                None => {
                    return FileResult {
                        name: String::new(),
                        passport: Vec::new(),
                        stamp: Vec::new(),
                        skip_reason: Some(format!("Skipping {:?}: invalid filename", path)),
                    };
                }
            };

            let ext = match path.extension().and_then(|s| s.to_str()) {
                Some(e) => e,
                None => {
                    return FileResult {
                        name: name.clone(),
                        passport: Vec::new(),
                        stamp: Vec::new(),
                        skip_reason: Some(format!("Skipping {}: unsupported format", name)),
                    };
                }
            };

            let format = match ImageFormat::from_extension(ext) {
                Some(f) => f,
                None => {
                    return FileResult {
                        name: name.clone(),
                        passport: Vec::new(),
                        stamp: Vec::new(),
                        skip_reason: Some(format!("Skipping {}: unsupported format", name)),
                    };
                }
            };

            let img = match image::open(path) {
                Ok(img) => img,
                Err(e) => {
                    return FileResult {
                        name: name.clone(),
                        passport: Vec::new(),
                        stamp: Vec::new(),
                        skip_reason: Some(format!("Skipping {}: corrupted ({})", name, e)),
                    };
                }
            };

            let (w, h) = img.dimensions();
            if w == 0 || h == 0 {
                return FileResult {
                    name: name.clone(),
                    passport: Vec::new(),
                    stamp: Vec::new(),
                    skip_reason: Some(format!("Skipping {}: zero-dimension image ({}x{})", name, w, h)),
                };
            }

            let squared = crop_to_square(&img);

            let passport = squared.resize_exact(600, 600, FilterType::Lanczos3);
            let stamp = squared.resize_exact(300, 300, FilterType::Lanczos3);

            let mut passport_buf = Cursor::new(Vec::new());
            if let Err(e) = passport.write_to(&mut passport_buf, format) {
                return FileResult {
                    name: name.clone(),
                    passport: Vec::new(),
                    stamp: Vec::new(),
                    skip_reason: Some(format!("Skipping {}: encode error ({})", name, e)),
                };
            }

            let mut stamp_buf = Cursor::new(Vec::new());
            if let Err(e) = stamp.write_to(&mut stamp_buf, format) {
                return FileResult {
                    name: name.clone(),
                    passport: Vec::new(),
                    stamp: Vec::new(),
                    skip_reason: Some(format!("Skipping {}: encode error ({})", name, e)),
                };
            }

            eprintln!("  Processed: {}", name);

            FileResult {
                name,
                passport: passport_buf.into_inner(),
                stamp: stamp_buf.into_inner(),
                skip_reason: None,
            }
        })
        .collect();

    let config = results.iter().fold(
        (0usize, 0usize, Vec::new()),
        |(mut processed, mut skipped, mut reasons), r| {
            match &r.skip_reason {
                Some(reason) => {
                    skipped += 1;
                    reasons.push(reason.clone());
                }
                None => processed += 1,
            }
            (processed, skipped, reasons)
        },
    );

    let (processed, skipped, skip_reasons) = config;

    if processed == 0 {
        eprintln!("No images processed successfully. Nothing to save.");
        for r in &skip_reasons {
            eprintln!("  {}", r);
        }
        std::process::exit(1);
    }

    let mut seen_names: HashSet<String> = HashSet::new();
    let mut deduped: Vec<(String, &[u8], &[u8])> = Vec::new();

    for result in &results {
        if result.skip_reason.is_some() {
            continue;
        }
        let mut unique_name = result.name.clone();
        let mut counter = 2;
        while seen_names.contains(&unique_name) {
            if let Some(dot) = result.name.rfind('.') {
                let base = &result.name[..dot];
                let ext = &result.name[dot..];
                unique_name = format!("{}_{}{}", base, counter, ext);
            } else {
                unique_name = format!("{}_{}", result.name, counter);
            }
            counter += 1;
        }
        seen_names.insert(unique_name.clone());
        deduped.push((unique_name, result.passport.as_slice(), result.stamp.as_slice()));
    }

    let file = match File::create(&output) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Cannot write to output path. Check permissions: {}", e);
            std::process::exit(1);
        }
    };

    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);

    for (name, passport, stamp) in &deduped {
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

    eprintln!("Done! Archive created: {}", output);
    eprintln!("Processed: {} images, Skipped: {} files", processed, skipped);
    if !skip_reasons.is_empty() {
        for r in &skip_reasons {
            eprintln!("  {}", r);
        }
    }
}