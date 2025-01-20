# Astronomical Image Noise Reduction

This project implements an A-Trous wavelet transform to reduce noise in astronomical images.

## Features
- Takes an astronomical image as input.
- Applies a multi-scale noise reduction process using A-Trous wavelet transform.
- Outputs a noise-reduced version of the image.

## Requirements
- **Rust**: Make sure you have Rust installed on your system. You can download it [here](https://www.rust-lang.org/tools/install).
- **Image**: You need an astronomical image in a supported format (e.g., `.jpg`, `.png`) as input.

## How to Use

1. **Prepare Your Image**:
   - Use an astronomical image (e.g., `m33-noise-lum.jpg`).
   - Move the image to the root directory of this project.

2. **Reference the Image in `main.rs`**:
   In `src/main.rs`, update the file name in the `image::open()` function to match your input file name:
   ```rust
   fn main() {
       // Open our noisy image
       let image = image::open("m33-noise-lum.jpg").unwrap();
   }
3. **Run the Program:**
 Open a terminal in the root directory of the project and execute:
    ```bash
    cargo run --release
 After the program finishes, a new file noise-reduced.jpg will be created in the root directory.
 This file contains the noise-reduced version of your input image.