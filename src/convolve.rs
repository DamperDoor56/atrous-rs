impl Convolution for ATrousTransform {
    fn compute_pixel_index(
        &self, 
        distance: usize, 
        kernel_index: [isize; 2], 
        target_pixel_index: [usize; 2]
    ) -> [usize; 2] {
        let [kernel_index_x, kernel_index_y] = kernel_index;

        // Compute the actual distance of adjacent pixel
        // by multiplying their relative position with the
        // size of the hole.
        let x_distance = kernel_index_x * distance as isize;
        let y_distance = kernel_index_y * distance as isize;

        let [x, y] = target_pixel_index;

        // Compute the index of adjacent pixel in the 2D
        // image based on the index of current pixel.
        let mut x = x as isize + x_distance;
        let mut y = y as isize + y_distance;

        // If x index is out of bounds, consider x to be
        // the nearest boundary location
        if x < 0 {
            x = 0;
        } else if x > self.width as isize - 1 {
            x = self.width as isize - 1;
        }

        // If y index is out of bounds, consider y to be
        // the nearest boundary location
        if y < 0 {
            y = 0;
        } else if y > self.height as isize - 1 {
            y = self.height as isize - 1;
        }

        // The final 2D index of pixel.
        [y as usize, x as usize]
    }

    fn compute_convoluted_pixel(
        &self, 
        distance: usize, 
        [x, y]: [usize; 2]
    ) -> f32 {
        // Create new variable to hold the result of convolution
        // for current pixel.
        let mut pixels_sum = 0.0;

        let kernel = LinearInterpolationKernel::default();

        // Iterate over relative position of pixels from the center
        // pixel to perform convolution with. In other words, 
        // these are the indexes of neighbouring pixels from the
        // center pixel.
        for kernel_index_x in -1..=1 {
            for kernel_index_y in -1..=1 {
                // Get the computed pixel location that maps to
                // the current position in kernel
                let pixel_index = self.compute_pixel_index(
                    distance,
                    [kernel_index_x, kernel_index_y],
                    [x, y]
                );

                // Get the multiplicative factor (kernel value) for 
                // this relative location from the kernel.
                let kernel_value = kernel.value_from_relative_index(
                    kernel_index_x,
                    kernel_index_y
                );

                // Multiply the pixel value with kernel scaling
                // factor and add it to the pixel sum.
                pixels_sum += kernel_value * self.input[pixel_index];
            }
        }

        // Return the value of computed pixel from convolution process.
        pixels_sum
    }
}