use image::{DynamicImage, ImageBuffer, Luma};
use ndarray::Array2;

pub trait RecomposableLayers: Iterator<Item = (Array2<f32>, Option<usize>)> {
    fn recompose_into_image(
        self,
        width: usize,
        height: usize,
    ) -> DynamicImage
        where
            Self: Sized,
    {
        // Create a result buffer to hold the pixel data for our output image.
        let mut result = Array2::<f32>::zeros((height, width));

        // For each layer, add the layer data to current value of result buffer.
        for layer in self {
            result += &layer.0;
        }

        // Compute min and max pixel intensity values in the final data so that
        // we can perform a "rescale", which normalizes all pixel values to be
        // between the range of 0 & 1, as is expected by float 32 images.
        let min_pixel = result.iter().copied().reduce(f32::min).unwrap();
        let max_pixel = result.iter().copied().reduce(f32::max).unwrap();

        // Create a new `ImageBuffer`, which is a type provided by `image` crate to
        // serve as buffer for pixel data of an image. Here, we're creating a new
        // `Luma` ImageBuffer with pixel value of type `u16`. Luma just refers to
        // grayscale.
        let mut result_img: ImageBuffer<Luma<u16>, Vec<u16>> =
            ImageBuffer::new(width as u32, height as u32);

        // Pre-compute the denominator for scaling computation so that we don't
        // repeat this unnecessarily for every iteration.
        let rescale_ratio = max_pixel - min_pixel;

        // Iterate over all pixels in the `ImageBuffer` and fill it based on data
        // from the `result` buffer after rescaling the value.
        for (x, y, pixel) in result_img.enumerate_pixels_mut() {
            let intensity = result[(y as usize, x as usize)];

            *pixel =
                Luma([((intensity - min_pixel) / rescale_ratio * u16::MAX as f32) as u16]);
        }

        // Convert the `ImageBuffer` into `DynamicImage` and return it
        DynamicImage::ImageLuma16(result_img)
    }
}

// Implement this trait for anything that implements the Iterator trait
// with the given item type
impl<T> RecomposableLayers for T where T: Iterator<Item = (Array2<f32>, Option<usize>)> {}