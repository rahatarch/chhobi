# Troubleshooting Guide

## "Nothing happened" / "No images found"

Chhobi can't find any supported photos in your folder.

**Check:**
- Does the folder exist? Type the full path to be sure.
- Are your photos `.jpg`, `.jpeg`, or `.png`? Other formats are ignored.
- Are the files inside the folder (not in a subfolder)?

**Fix:**
```bash
chhobi --input "C:\Users\You\Desktop\my-photos" --output "resized.zip"
```

## "Some photos are missing from the output"

A few photos didn't make it into the ZIP.

**Possible causes:**
- The file is a format Chhobi doesn't support (GIF, BMP, TIFF, HEIC, etc.)
- The file is corrupted or damaged
- The file name has unusual characters

**Fix:** Convert unsupported photos to JPG or PNG using any free online converter, then run Chhobi again.

## "The ZIP file wasn't created"

Chhobi says "Cannot create output file" or similar.

**Possible causes:**
- You don't have permission to write to the current folder
- A file with the same name already exists and is locked

**Fix:**
- Try running the command from a different folder (like your Desktop)
- Close any program that might have the ZIP file open
- Make sure the output path is writable:

```bash
chhobi --input "photos" --output "C:\Users\You\Desktop\resized.zip"
```

## "Photos look stretched or distorted"

Chhobi always crops to a **square first**, then resizes. Photos are never stretched. If a photo looks unusual, it's because the original was very wide or tall and most of the image was cropped away to make it square.

**What to do:** If you need the full image without cropping, Chhobi may not be the right tool for your use case. Consider using a photo editor that preserves the original aspect ratio.

## "Command not found"

Your computer doesn't know where Chhobi is.

**Fix:** Navigate to the folder where you saved Chhobi, then run it with `./chhobi` (macOS/Linux) or `.\chhobi.exe` (Windows).

## Still Stuck?

Check that you're using the latest version of Chhobi. If the problem continues, please open an issue on the project page with:
- Your operating system (Windows 10, macOS Sonoma, Ubuntu 22.04, etc.)
- The exact command you ran
- The full error message (if any)