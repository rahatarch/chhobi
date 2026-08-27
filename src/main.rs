use std::path::Path;

use rayon::prelude::*;

fn main() {
    let (input, output) = chhobi::cli::parse_args();
    let input_path = Path::new(&input);

    let (paths, unsupported) = chhobi::pipeline::walk_images(input_path);
    let unsupported_count = unsupported.len();

    if paths.is_empty() {
        eprintln!("No images found in {}", input);
        std::process::exit(1);
    }

    for name in &unsupported {
        eprintln!("Ignoring unsupported file: {}", name);
    }
    eprintln!("Found {} images. Processing...", paths.len());
    if unsupported_count > 0 {
        eprintln!("Ignoring {} unsupported file(s).", unsupported_count);
    }

    let results: Vec<chhobi::pipeline::FileResult> = paths
        .par_iter()
        .map(|path| chhobi::pipeline::process_file(path))
        .collect();

    let summary = chhobi::pipeline::aggregate_results(&results);

    if summary.processed == 0 {
        eprintln!("No images processed successfully. Nothing to save.");
        for r in &summary.skip_reasons {
            eprintln!("  {}", r);
        }
        std::process::exit(1);
    }

    let deduped = chhobi::pipeline::dedup_results(&results);
    chhobi::archive::create_zip(&output, &deduped);
    chhobi::cli::print_summary(summary.processed, summary.skipped, unsupported_count, &summary.skip_reasons, &output);
}