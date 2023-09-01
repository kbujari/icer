pub mod transform;

use crate::Icer;
use image::{ImageBuffer, Luma};

pub fn wavelet_transform(config: &Icer, data: &ImageBuffer<Luma<u8>, Vec<u8>>) {
    for row in data.rows() {
        transform::transform_1d(row, &config.filter_params);
    }
}
