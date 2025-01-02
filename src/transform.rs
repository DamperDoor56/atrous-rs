
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