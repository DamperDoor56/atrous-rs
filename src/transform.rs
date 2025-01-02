
use ndarray::Array2;

pub struct ATrousTransform {
    input: Array2<f32>, // `Array2<f32>` is a 2D array where each value is of type `f32`. This will hold our pixel data for input image.
    levels: usize, // The number of levels or scales to decompose the image into
    current_level: usize, // Current level that we need to generate. This holds the state of our iterator.
    width: usize, // Width of input image
    height: usize, // Height of input image
}
