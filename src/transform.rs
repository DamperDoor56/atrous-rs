
use ndarray::Array2;
use image::GenericImageView;

pub struct ATrousTransform {
    input: Array2<f32>, // `Array2<f32>` is a 2D array where each value is of type `f32`. This will hold our pixel data for input image.
    levels: usize, // The number of levels or scales to decompose the image into
    current_level: usize, // Current level that we need to generate. This holds the state of our iterator.
    width: usize, // Width of input image
    height: usize, // Height of input image
}


impl ATrousTransform {
    pub fn new(input: &image::DynamicImage, levels: usize) -> Self {
        let (width, height) = input.dimensions();
        let (width, height) = (width as usize, height as usize);

        // Create a new 2D array with proper size for each dimension to hold all of our input's pixel data. Method `zeros` takes a "shape" parameter, which is a tuple of (rows_count, columns_count).
        let mut data = Array2::<f32>::zeros((height, width));

        // Convert the image to be a grayscale image where each pixel value is of type `f32`. Loop over all pixels in the input image along with its 2D location.
        for (x, y, pixel) in input.to_luma32f().enumerate_pixels() {
            // Put the pixel value at appropriate location in our data array. The `[[]]` syntax is used to provide a 2-dimensional index such as `[[row_index, col_index]]`
            data[[y as usize, x as usize]] = pixel.0[0];
        }

        Self {
            input: data,
            levels,
            current_level: 0,
            width,
            height
        }
    }
}

impl Iterator for ATrousTransform {
    // Our output is an image as well as the current level for each
    // iteration. The current level is an `Option` to represent the
    // final residue layer after the intermediary layers have been
    // generated.
    type Item = (Array2::<f32>, Option<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        let pixel_scale = self.current_level;
        self.current_level += 1;

        // We've already generated all the layers. Return None to 
        // exit the iterator.
        if pixel_scale > self.levels {
            return None;
        }

        // We've generated all intermediary layers, return the 
        // residue layer.
        if pixel_scale == self.levels {
            return Some((self.input.clone(), None))
        }

        let (width, height) = (self.width, self.height);

        // Distance between adjacent pixels for convolution (also 
        // referred to as size of "hole").
        let distance = 2_usize.pow(pixel_scale as u32);

        // Create new buffer to hold the computed data for this layer.
        let mut current_data = Array2::<f32>::zeros((height, width));

        // Iterate over each pixel location in the 2D image
        for x in 0..width {
            for y in 0..height {
                // Set the current pixel in current layer to
                // the result of convolution on the current
                // pixel in input data.
                current_data[[y, x]] = self.compute_convoluted_pixel(
                    distance, 
                    [x, y]
                );
            }
        }

        // Create current layer by subtracting currently computed pixels 
        // from previous layer
        let final_data = self.input.clone() - &current_data;

        // Set the input layer to equal the current computed layer so 
        // that it can be used as the "previous layer" in next iteration.
        // This is also our residue data for each layer.
        self.input = current_data;

        // Return the current layer data as well as current level information.
        Some((final_data, Some(self.current_level)))
    }
}