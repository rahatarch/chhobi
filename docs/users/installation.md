# Installation Guide

## What You Need

- A computer running **Windows**, **macOS**, or **Linux**
- The **Chhobi** program (one file, nothing else)

## How to Install

### Option 1: Download the executable (recommended)

1. Go to the [Chhobi releases page](https://github.com/your-org/chhobi/releases)
2. Find the latest version (e.g. `v0.0.1-beta`)
3. Download the file for your system:
   - **Windows**: `chhobi-windows.exe`
   - **macOS**: `chhobi-macos`
   - **Linux**: `chhobi-linux`
4. Move the downloaded file to a folder you can find later (like `Desktop` or `Documents`)

### Option 2: Install from local source code (for advanced users)

If you have Rust installed, you can install Chhobi directly from the source folder. **Minimum Rust version: 1.70.0** (check with `rustc --version`).

1. Open a terminal and navigate to the **chhobi** folder (the one containing `Cargo.toml`):

```bash
cd /path/to/chhobi
```

2. Install it from the local source:

```bash
cargo install --path .
```

3. Verify it's installed:

```bash
chhobi --version
```

This installs Chhobi from the code on your computer, not from the internet. Once installed, the `chhobi` command will be available globally in any terminal.

## How to Uninstall

- **If you installed from source:** Run `cargo uninstall chhobi` in any terminal.
- **If you downloaded the executable:** Simply delete the file.

## How to Verify It's Installed

Open a terminal and run:

```bash
chhobi --help
```

If you see a message showing how to use the tool, you're all set.

> **If you downloaded the executable:** Run it from the folder where you saved it, using `./chhobi --help` (macOS/Linux) or `chhobi.exe --help` (Windows).  
> **If you installed from source:** The `chhobi` command is available globally — run it from any folder.