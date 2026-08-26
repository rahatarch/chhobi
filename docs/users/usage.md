# Usage Guide

## The Basic Command

Open a terminal and run:

```bash
chhobi --input "path/to/photo/folder" --output "my-photos.zip"
```

## What Each Flag Does

| Flag | Purpose |
|------|---------|
| `--input` | The folder containing your original photos |
| `--output` | The name of the ZIP file to create |
| `--help` | Show all available options and usage information |
| `--version` | Display the current version of Chhobi |

## Real Example

Let's say you have a folder called `passport-photos` on your Desktop:

**On Windows:**
```bash
chhobi --input "C:\Users\YourName\Desktop\passport-photos" --output "passport-sizes.zip"
```

**On macOS / Linux:**
```bash
chhobi --input "/Users/yourname/Desktop/passport-photos" --output "passport-sizes.zip"
```

## What Happens When You Run It

1. Chhobi scans the folder for all `.jpg`, `.jpeg`, and `.png` files
2. You'll see a message like: `Found 45 images. Processing...`
3. Each photo is cropped, resized, and added to the ZIP
4. When done, you'll see: `Done! Archive created: passport-sizes.zip`

## Where the ZIP File Goes

The ZIP file is created in the **same folder where you ran the command**. 

If you ran:

```bash
chhobi --input "Desktop/photo-folder" --output "output.zip"
```

The ZIP file will appear in your current folder, not inside `photo-folder`.

## What's Inside the ZIP

Open the ZIP file and you'll see two folders:

- **`passport/`** — 600×600 versions of every photo
- **`stamp/`** — 300×300 versions of every photo

## Tips

- **Processing 500+ photos?** Chhobi uses all your computer's cores and will still finish in seconds
- **If output.zip already exists,** Chhobi will overwrite it without asking.
- **If an image is corrupted or fails to process,** Chhobi skips it and continues with the rest. A warning message will show which file was skipped.