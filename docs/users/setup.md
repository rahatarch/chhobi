# Setup Guide

## Before You Run Chhobi

Chhobi needs two things: a **folder of photos** and a **name for the output ZIP file**.

## Step 1: Organise Your Photos

Put all the photos you want to resize into one folder. You can name it anything — for example:

```
Desktop/
└── school-photos/
    ├── student-01.jpg
    ├── student-02.jpg
    ├── student-03.png
    └── ...
```

## Step 2: Supported Formats

Chhobi works with these file types:

| Format | Extension | Example |
|--------|-----------|---------|
| JPEG | `.jpg` or `.jpeg` | `photo.jpg` |
| PNG | `.png` | `photo.png` |

All other file types (GIF, BMP, TIFF, PDF, etc.) are **skipped** automatically.

## Step 3: What to Expect

Chhobi does three things to every photo:

1. **Crops to a square** — takes the centre of the image so nothing important is cut off
2. **Creates a passport size** — 600 pixels wide by 600 pixels tall
3. **Creates a stamp size** — 300 pixels wide by 300 pixels tall

## Important Notes

- **Your original photos are never changed.** Chhobi only reads them.
- **The output folder must be writable.** Make sure you have permission to create files there.
- **Chhobi only scans the folder you specify.** It does not look inside subfolders.