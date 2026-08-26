# How It Works

## Overview

Chhobi is a single-binary CLI tool written in Rust. It reads a directory of images, crops each to a square, produces two resized variants (passport 600×600, stamp 300×300), and packages everything into a ZIP archive.

The entire program is a straight-line pipeline — no config files, no plugins, no daemon.

## Pipeline

```
Input Directory
    │
    ▼
┌─────────────────────────────────┐
│  File Discovery (ignore::Walk)  │
│  Filters: .jpg, .jpeg, .png     │
└─────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────┐
│  Parallel Processing (rayon)    │
│  ┌───────────────────────────┐  │
│  │ Per-image:                │  │
│  │   1. Decode (image::open) │  │
│  │   2. Crop to square       │  │
│  │   3. Resize to 600×600    │  │
│  │   4. Resize to 300×300    │  │
│  │   5. Encode to bytes      │  │
│  └───────────────────────────┘  │
└─────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────┐
│  ZIP Assembly (zip crate)      │
│  Structure:                    │
│    passport/<name>.jpg         │
│    stamp/<name>.jpg            │
└─────────────────────────────────┘
    │
    ▼
Output ZIP File
```

## Current File Structure

```
chhobi/
├── Cargo.toml          # Package manifest; 4 dependencies
├── Cargo.lock          # Lockfile (auto-generated)
├── docs/
│   ├── roadmap.md      # Upcoming features
│   ├── users/          # End-user documentation
│   └── engineers/      # This directory
└── src/
    └── main.rs         # Entire program (136 lines)
```

### `src/main.rs` — The Whole Program

| Lines | Section | Purpose |
|-------|---------|---------|
| 1–12 | Imports | `image`, `rayon`, `zip`, `ignore` crates + stdlib |
| 14–20 | `crop_to_square()` | Pure function: center-crops a `DynamicImage` to `min(w,h)` |
| 22–136 | `main()` | CLI parsing, discovery, processing, ZIP assembly |

There are no helper modules, no error types, no config structs. All error handling is `Option`/`Result` with `expect()` or `eprintln!` + `exit(1)`.

## Code Walkthrough

### Imports (`src/main.rs:1-12`)

```rust
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
```

- `Cursor<Vec<u8>>` is used as an in-memory writer for encoded image bytes (avoids temp files).
- `GenericImageView` is the trait that provides `.dimensions()` on `DynamicImage`.
- `FilterType::Lanczos3` is a high-quality resampling filter (sharp, suitable for photos).
- `rayon::prelude::*` brings `par_iter()` into scope.

### `crop_to_square()` (`src/main.rs:14-20`)

```rust
fn crop_to_square(img: &DynamicImage) -> DynamicImage {
    let (w, h) = img.dimensions();
    let size = w.min(h);
    let x = (w - size) / 2;
    let y = (h - size) / 2;
    img.crop_imm(x, y, size, size)
}
```

The crop is **center-weighted**: it takes the smaller dimension as the square size, then offsets by half the difference on the larger axis. For a 1200×800 image, `size = 800`, `x = (1200 - 800) / 2 = 200`, `y = 0`. The result is the center 800×800 region.

`crop_imm` is the immutable variant — it returns a new `DynamicImage` without modifying the original.

### Argument Parsing (`src/main.rs:22-67`)

The program parses `--input`, `--output`, `--help`/`-h`, and `--version`/`-V` from `std::env::args()` manually (no `clap` dependency):

```rust
let input = match args.iter().position(|a| a == "--input").and_then(|i| args.get(i + 1)) {
    Some(v) => v.clone(),
    None => { eprint!("{}", help); std::process::exit(1); }
};
```

If `--input` is missing or the next argument doesn't exist, it prints help and exits with code 1. The input path is validated with `.is_dir()`.

### File Discovery (`src/main.rs:69-85`)

```rust
let paths: Vec<_> = ignore::WalkBuilder::new(input_path)
    .standard_filters(false)
    .build()
    .filter_map(|entry| { ... })
    .collect();
```

`ignore::WalkBuilder` recursively walks the directory. With `standard_filters(false)`, it does NOT skip hidden files or `.gitignore`-listed files. The `filter_map` closure:
- Skips non-files (directories, symlinks)
- Checks the extension is `jpg`, `jpeg`, or `png` (case-insensitive)
- Collects matching paths into a `Vec<PathBuf>`

If no images are found, the program exits with an error.

### Parallel Processing (`src/main.rs:94-117`)

```rust
let results: Vec<(String, Vec<u8>, Vec<u8>)> = paths
    .par_iter()
    .filter_map(|path| { ... })
    .collect();
```

`rayon::par_iter()` splits the path vector across a thread pool (typically one per CPU core). Each thread:

1. **Decodes** the image: `image::open(path)` — this detects the format automatically from the file header.
2. **Crops**: `crop_to_square(&img)` — center-crop to square.
3. **Resizes**: `squared.resize_exact(600, 600, FilterType::Lanczos3)` — produces a 600×600 passport variant. `resize_exact` ignores aspect ratio because we already cropped to square.
4. **Resizes again**: same for 300×300 stamp variant.
5. **Encodes**: each variant is written to a `Cursor<Vec<u8>>` via `.write_to(&mut buf, format)` using the original image format.

The result is a tuple `(filename, passport_bytes, stamp_bytes)`.

**Why `filter_map` instead of `map`?** If any image fails to decode or encode, the `filter_map` returns `None` and that image is silently skipped. A progress line is printed to stderr for each successfully processed image.

### ZIP Assembly (`src/main.rs:119-135`)

```rust
let file = File::create(&output).expect("Cannot create output file");
let mut zip = ZipWriter::new(file);
let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
```

`CompressionMethod::Stored` means no compression. JPEG and PNG are already compressed; re-compressing them with Deflate would waste CPU time with negligible size savings.

The loop writes each result into the ZIP:

```rust
for (name, passport, stamp) in &results {
    zip.start_file(format!("passport/{}", name), options)
        .expect("Cannot write passport entry to zip");
    zip.write_all(passport).expect("Cannot write passport data to zip");

    zip.start_file(format!("stamp/{}", name), options)
        .expect("Cannot write stamp entry to zip");
    zip.write_all(stamp).expect("Cannot write stamp data to zip");
}
```

Each image produces two entries in the ZIP: `passport/<filename>` and `stamp/<filename>`. `zip.start_file()` creates a new entry header; `zip.write_all()` writes the raw bytes. Both use `expect()` because writing to a file handle that we just successfully created should not fail under normal conditions.

`zip.finish()` finalizes the archive by writing the central directory and end-of-central-directory record.

## Design Decisions

### 1. Crop before resize

Cropping to a square first ensures the aspect ratio is 1:1 before scaling. If we resized first, the image would be distorted (squashed/stretched) to fit 600×600 or 300×300. The crop is center-weighted: it takes `min(w, h)` from the center of the image.

### 2. Parallel processing with rayon

Image encoding is CPU-bound. `rayon::par_iter()` distributes images across all available cores. The bottleneck is the encode step (`image::DynamicImage::write_to`), which is trivially parallelizable because each image is independent.

### 3. ZIP with `Stored` compression

Images are already compressed (JPEG is lossy, PNG is lossless). Re-compressing them with Deflate is wasteful. The code uses `CompressionMethod::Stored` (store-only) for speed.

### 4. Single-file binary

The entire program is 136 lines in `src/main.rs`. There is no library crate, no separate modules — everything lives in one file. This is intentional for a tool of this scope.

### 5. `ignore::WalkBuilder` over `std::fs::read_dir`

The `ignore` crate respects `.gitignore` (when `standard_filters(true)` is used) and handles symlinks, permissions, and hidden files more robustly than a manual `read_dir`. Currently `standard_filters` is false to avoid skipping `.gitignore`-listed files inadvertently.

### 6. `eprintln!` for progress, `stdout` for data

Progress messages go to stderr so piping or redirecting stdout (future use) won't mix status text with output data.

## CLI Flags

| Flag | Short | Description |
|------|-------|-------------|
| `--input` | — | Directory containing JPG/PNG images (required) |
| `--output` | — | Path for the output ZIP archive (required) |
| `--help` | `-h` | Show help message and exit |
| `--version` | `-V` | Show version information and exit |

## Known Limitations

### Silent failure on corrupt images

If an image fails to decode or encode, it is silently skipped. A progress line is printed to stderr only for successfully processed images. There is no warning when individual images are dropped.

### Empty results

If all images fail during processing, `results` will be empty and the program will produce a ZIP archive with no entries. The program does not currently check for this case.

### Minimum image size

Images smaller than 600×600 are upscaled to 600×600 (passport) and 300×300 (stamp) using `Lanczos3` filtering. This may produce blurry results for very small source images. For best quality, provide images at least 600×600 pixels.

## Error Handling Philosophy

The code uses two patterns:
- **`expect()`** for operations that should never fail in normal conditions (file creation, writing to an open file handle).
- **`filter_map` + `None`** for operations that can legitimately fail per-image (corrupted file, unsupported format). This allows the batch to continue when individual images are broken.
- **`eprintln!` + `exit(1)`** for fatal errors that should abort the entire program (bad arguments, empty input directory, cannot create output file).