#[derive(Copy, Clone)]
pub struct LinearInterpolationKernel {
    values: [[f32; 3]; 3]
}

impl Default for LinearInterpolationKernel {
    fn default() -> Self {
        Self {
            values: [
                [1. / 16., 1. / 8., 1. / 16.],
                [1. / 8., 1. / 4., 1. / 8.],
                [1. / 16., 1. / 8., 1. / 16.],
            ]
        }
    }
}