use std::collections::HashSet;
use std::path::Path;

use image::GenericImageView;
use image::ImageFormat;

use crate::image_ops::{crop_to_square, encode_image, resize_passport, resize_stamp};

pub struct FileResult {
    pub name: String,
    pub passport: Vec<u8>,
    pub stamp: Vec<u8>,
    pub skip_reason: Option<String>,
}

pub struct ProcessingSummary {
    pub processed: usize,
    pub skipped: usize,
    pub skip_reasons: Vec<String>,
}

pub fn walk_images(input_path: &Path) -> (Vec<std::path::PathBuf>, Vec<String>) {
    let mut unsupported: Vec<String> = Vec::new();

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
                if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                    unsupported.push(name.to_string());
                }
                None
            }
        })
        .collect();

    (paths, unsupported)
}

pub fn process_file(path: &Path) -> FileResult {
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
    let passport_img = resize_passport(&squared);
    let stamp_img = resize_stamp(&squared);

    let passport_bytes = match encode_image(&passport_img, format) {
        Ok(b) => b,
        Err(e) => {
            return FileResult {
                name: name.clone(),
                passport: Vec::new(),
                stamp: Vec::new(),
                skip_reason: Some(format!("Skipping {}: {}", name, e)),
            };
        }
    };

    let stamp_bytes = match encode_image(&stamp_img, format) {
        Ok(b) => b,
        Err(e) => {
            return FileResult {
                name: name.clone(),
                passport: Vec::new(),
                stamp: Vec::new(),
                skip_reason: Some(format!("Skipping {}: {}", name, e)),
            };
        }
    };

    eprintln!("  Processed: {}", name);

    FileResult {
        name,
        passport: passport_bytes,
        stamp: stamp_bytes,
        skip_reason: None,
    }
}

pub fn dedup_results(results: &[FileResult]) -> Vec<(String, &[u8], &[u8])> {
    let mut seen_names: HashSet<String> = HashSet::new();
    let mut deduped: Vec<(String, &[u8], &[u8])> = Vec::new();

    for result in results {
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

    deduped
}

pub fn aggregate_results(results: &[FileResult]) -> ProcessingSummary {
    let mut processed = 0usize;
    let mut skipped = 0usize;
    let mut skip_reasons: Vec<String> = Vec::new();

    for r in results {
        match &r.skip_reason {
            Some(reason) => {
                skipped += 1;
                skip_reasons.push(reason.clone());
            }
            None => processed += 1,
        }
    }

    ProcessingSummary { processed, skipped, skip_reasons }
}