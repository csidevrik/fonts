# Font Collection Repository

This repository contains a curated collection of fonts organized by their primary intended use or platform compatibility.




## Directory Structure

The fonts are organized in directories with the following naming convention:

- `2linux/` - Fonts compatible with Linux systems (Size: 7.72 MB)
- `2word/` - Fonts optimized for Microsoft Word and Office applications (Size: 8.51 MB)

## Current Collections

### 2word
This directory contains fonts specifically tested and optimized for use with Microsoft Word and other Office applications. These fonts ensure proper rendering and compatibility within the Microsoft Office ecosystem.

## Adding New Fonts

When adding new fonts to this repository, please place them in the appropriate directory based on their primary intended use or platform compatibility.

## Note

The "2" prefix in directory names represents "for" or "to" (e.g., 2word = for Word, 2linux = for Linux).

## Usage Instructions

To update this README with current directory information:

1. Open a terminal in the repository root
2. Run the program using either:
   ```bash
   cargo run
   ```
   or
   ```bash
   cargo build --release
   ./target/release/runrr.exe
   ```

The program will automatically:
- Scan for directories starting with "2"
- Calculate their sizes
- Update the Directory Structure section above
