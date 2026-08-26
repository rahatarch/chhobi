# Release Notes

## v0.0.1-beta — August 2026

Initial public release of Chhobi.

### What's New

- **Bulk image resizing** — point Chhobi at a folder of photos and get all of them resized in seconds
- **Smart square crop** — every photo is automatically cropped from the centre, so nothing important is cut off
- **Two sizes per photo** — each image becomes a 600×600 passport and a 300×300 stamp
- **ZIP output** — all resized images are packed into a single ZIP file, organised into `passport/` and `stamp/` folders
- **Parallel processing** — uses all your computer's cores to handle hundreds of photos quickly
- **Supports JPG and PNG** — the most common photo formats

### Known Limitations

- Only JPG and PNG files are supported (GIF, BMP, TIFF, and HEIC are not yet supported)
- All images are cropped to a square — non-square originals will lose some content
- Command-line only (no graphical interface yet)

### What's Coming Next

- HEIC (iPhone) format support
- Custom output sizes
- A simple graphical interface
- Drag-and-drop support