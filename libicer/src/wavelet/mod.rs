pub mod transform;

use crate::Icer;
use image::{ImageBuffer, Luma};

pub fn wavelet_transform(config: &Icer, data: &ImageBuffer<Luma<u8>, Vec<u8>>) {
    for row in data.rows() {
        let half = row.len() / 2;
        let is_odd = row.len() & 1 == 1;

        let input: Vec<u8> = row.skip(half).map(|el| *el.0.first().unwrap()).collect();

        transform::transform_1d(input.as_slice(), &config.filter_params, is_odd);
    }
}
