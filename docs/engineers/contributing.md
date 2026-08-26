# Contributing

## Prerequisites

| Tool | Minimum Version | Reason |
|------|----------------|--------|
| Rust | 1.85+ (edition 2024) | Project uses `edition = "2024"` in `Cargo.toml` |
| Cargo | Bundled with Rust | Build, test, dependency management |

Check your Rust version:

```bash
rustc --version    # Should be >= 1.85
```

If you're on an older toolchain, update:

```bash
rustup update stable
```

## Clone & Build

```bash
git clone https://github.com/<your-org>/chhobi.git
cd chhobi

# Debug build (fast iteration)
cargo build

# Release build (optimized, ~2MB binary)
cargo build --release

# Run directly
cargo run -- --input "test-images" --output "out.zip"
```

The release binary lives at `target/release/chhobi`.

## Test

```bash
# Run all tests
cargo test

# Check without building (fast)
cargo check

# Lint
cargo clippy

# Format
cargo fmt
```

## Project Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `image` | 0.25 | Image decoding, encoding, cropping, resizing |
| `rayon` | 1.12 | Parallel iteration over images |
| `zip` | 8.6 | ZIP archive creation |
| `ignore` | 0.4 | File discovery / directory walking |

These are defined in `Cargo.toml`.

## Code Style

- **Format**: `cargo fmt` before committing. The project uses default Rust formatting.
- **Clippy**: `cargo clippy` must pass with no warnings.
- **Naming**: Standard Rust conventions (`snake_case` for functions/vars, `CamelCase` for types).
- **Error handling**: Prefer `anyhow` or `thiserror` for new error types. Keep `unwrap()`/`expect()` only where failure is truly unrecoverable (e.g., writing to a file handle we just created).
- **Comments**: Explain *why*, not *what*. The code should be self-documenting for the *what*.
- **Scope**: Keep the single-binary approach until the codebase justifies modularization.

## Testing

### Unit tests

Add `#[cfg(test)] mod tests` blocks at the bottom of the file being tested. For `crop_to_square`, test with known dimensions:

```rust
#[test]
fn test_crop_to_square_landscape() {
    let img = DynamicImage::new_rgba8(800, 600);
    let cropped = crop_to_square(&img);
    assert_eq!(cropped.dimensions(), (600, 600));
}
```

### Integration tests

Create `tests/` directory with end-to-end tests that run the binary against a temporary directory of test images and verify the output ZIP structure.

### Running tests

```bash
cargo test          # Run all tests
cargo test -- --nocapture  # Show stdout/stderr
```

## Test Images

For manual testing, create a directory with a few JPG/PNG files:

```bash
mkdir test-images
# Copy or generate test images
cargo run -- --input test-images --output test-output.zip
unzip -l test-output.zip  # Verify structure
```

## How to Contribute

1. **Open an issue** first for bugs or feature requests.
2. **Fork** the repo, create a feature branch (`feat/description` or `fix/description`).
3. **Write code** following the style guide above.
4. **Run `cargo check`** and **`cargo test`** (once tests exist).
5. **Open a PR** with a clear title and description linking to the issue.

## PR Guidelines

- **One feature per PR**. Small, focused PRs are reviewed faster.
- **Keep the diff small**. No whitespace changes, no unrelated refactoring.
- **Update docs** if the CLI flags, output format, or behavior changes.
- **Update `docs/roadmap.md`** if you're implementing a roadmap item.

## Release Process

1. Bump version in `Cargo.toml`.
2. Update `docs/users/release-notes.md`.
3. Tag the commit (`v0.0.2`).
4. Build with `cargo build --release`.
5. Upload the binary to the GitHub release.