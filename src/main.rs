use image::{DynamicImage, ImageBuffer, Luma};
use atrous::recompose::RecomposableLayers;
use atrous::transform::ATrousTransform;

fn main() {
    // Reference your image here!:
    let image = image::open("image.png").unwrap();

    // Create a new instance of the transform with 9 layers
    let transform = ATrousTransform::new(&image, 9);

    // Map over each layer
    transform.map(|(mut buffer, pixel_scale)| {
        // Create a new image buffer to hold the pixel data. This
        // will be populated from the raw buffer for this layer.
        let mut new_buffer =
            ImageBuffer::<Luma<u16>, Vec<u16>>::new(buffer.ncols() as u32, buffer.nrows() as u32);

        // Iterate over all pixels of the `ImageBuffer` to populate it. We also
        // convert from `f32` pixels to `u16` pixels.
        for (x, y, pixel) in new_buffer.enumerate_pixels_mut() {
            *pixel = Luma([(buffer[[y as usize, x as usize]] * u16::MAX as f32) as u16])
        }

        // If the present layer is a small scale layer (< 3), 
        // perform noise reduction
        if pixel_scale.is_some_and(|scale| scale < 3) {
            let mut image = DynamicImage::ImageLuma16(new_buffer).to_luma8();

            // Bilateral filter is a de-noising filter. Apply it to the image.
            image = imageproc::filter::bilateral_filter(&image, 10, 10., 3.);

            // Modify the raw buffer to contain the updated pixel values after
            // filtering.
            for (x, y, pixel) in image.enumerate_pixels() {
                buffer[[y as usize, x as usize]] = pixel.0[0] as f32 / u8::MAX as f32;
            }

            // Return the updated buffer.
            (buffer, pixel_scale)
        } else {
            // Return the unmodified buffer for larger scale layers.
            (buffer, pixel_scale)
        }
    })
        // Call the recomposition method on iterator
        .recompose_into_image(image.width() as usize, image.height() as usize)
        // Convert output to 8-bit grayscale image
        .to_luma8()
        // Save it to jpg file
        .save("noise-reduced.jpg")
        .unwrap()
}