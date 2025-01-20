
use ndarray::Array2;
use image::GenericImageView;


pub struct ATrousTransform {
    input: Array2<f32>, // 2D array to hold pixel data for the input image.
    levels: usize, // Number of levels or scales to decompose the image into.
    current_level: usize, // Current level being generated in the iterator.
    width: usize, // Width of the input image.
    height: usize, // Height of the input image.
}

impl ATrousTransform {
    /// Create a new ATrousTransform instance.
    pub fn new(input: &image::DynamicImage, levels: usize) -> Self {
        let (width, height) = input.dimensions();
        let (width, height) = (width as usize, height as usize);

        // Create a new 2D array to hold all pixel data of the input image.
        let mut data = Array2::<f32>::zeros((height, width));

        // Convert the image to grayscale and populate the array with pixel values.
        for (x, y, pixel) in input.to_luma32f().enumerate_pixels() {
            data[[y as usize, x as usize]] = pixel.0[0];
        }

        Self {
            input: data,
            levels,
            current_level: 0,
            width,
            height,
        }
    }

    /// Compute the convoluted pixel value for a given coordinate and distance.
    pub fn compute_convoluted_pixel(&self, distance: usize, coord: [usize; 2]) -> f32 {
        let (x, y) = (coord[0], coord[1]);
        let (width, height) = (self.width, self.height);

        // Define a simple 3x3 kernel for convolution.
        let kernel: [[f32; 3]; 3] = [
            [0.1, 0.8, 0.1],
            [0.8, 1.0, 0.8],
            [0.1, 0.8, 0.1],
        ];

        let mut sum = 0.0;
        let mut weight = 0.0;

        // Apply convolution over the kernel.
        for ky in 0..3 {
            for kx in 0..3 {
                let nx = x as isize + (kx as isize - 1) * distance as isize;
                let ny = y as isize + (ky as isize - 1) * distance as isize;

                // Ensure the coordinates are within the image bounds.
                if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                    let pixel_value = self.input[[ny as usize, nx as usize]];
                    let kernel_value = kernel[ky][kx];

                    sum += pixel_value * kernel_value;
                    weight += kernel_value;
                }
            }
        }

        // Normalize the pixel value by the total weight of the kernel.
        if weight > 0.0 {
            sum / weight
        } else {
            0.0
        }
    }
}

impl Iterator for ATrousTransform {
    type Item = (Array2<f32>, Option<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        let pixel_scale = self.current_level;
        self.current_level += 1;

        // Exit the iterator if all layers have been generated.
        if pixel_scale > self.levels {
            return None;
        }

        // Return the residue layer for the final iteration.
        if pixel_scale == self.levels {
            return Some((self.input.clone(), None));
        }

        let (width, height) = (self.width, self.height);

        // Compute the distance between adjacent pixels for this layer.
        let distance = 2_usize.pow(pixel_scale as u32);

        // Create a new buffer to hold the computed layer data.
        let mut current_data = Array2::<f32>::zeros((height, width));

        // Populate the buffer by computing convoluted values for each pixel.
        for x in 0..width {
            for y in 0..height {
                current_data[[y, x]] = self.compute_convoluted_pixel(distance, [x, y]);
            }
        }

        // Calculate the final layer data by subtracting the computed layer from the input.
        let final_data = self.input.clone() - &current_data;

        // Update the input layer for the next iteration.
        self.input = current_data;

        // Return the current layer data and its scale level.
        Some((final_data, Some(self.current_level)))
    }
}
