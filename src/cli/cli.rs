use std::path::Path;

pub const HELP: &str = r"chhobi — batch crop and resize images for passport/stamp printing

Usage:
  chhobi --input <folder> --output <zip-file>

Flags:
  --input   <folder>     Directory containing JPG/PNG images
  --output  <zip-file>   Path for the output ZIP archive
  --help                 Show this help message and exit
  --version              Show version information and exit
";

pub fn parse_args() -> (String, String) {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        print!("{}", HELP);
        std::process::exit(0);
    }

    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("chhobi {}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }

    let input = match args.iter().position(|a| a == "--input").and_then(|i| args.get(i + 1)) {
        Some(v) => v.clone(),
        None => {
            eprint!("{}", HELP);
            std::process::exit(1);
        }
    };

    let output = match args.iter().position(|a| a == "--output").and_then(|i| args.get(i + 1)) {
        Some(v) => v.clone(),
        None => {
            eprint!("{}", HELP);
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

    (input, output)
}

pub fn print_summary(processed: usize, skipped: usize, unsupported: usize, skip_reasons: &[String], output: &str) {
    eprintln!("Done! Archive created: {}", output);
    eprintln!("Processed: {} images, Skipped: {} files, Unsupported: {} files", processed, skipped, unsupported);
    if !skip_reasons.is_empty() {
        for r in skip_reasons {
            eprintln!("  {}", r);
        }
    }
}