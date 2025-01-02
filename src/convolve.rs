pub trait Convolution {
    fn compute_pixel_index(
        &self,
        distance: usize,
        kernel_index: [isize; 2],
        target_pixel_index: [usize; 2]
    ) -> [usize; 2];

    fn compute_convoluted_pixel(
        &self, 
        distance: usize, 
        index: [usize; 2]
    ) -> f32;
}