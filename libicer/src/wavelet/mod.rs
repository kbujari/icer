pub mod transform;

use crate::Icer;
use image::{ImageBuffer, Luma};

pub fn wavelet_transform(config: &Icer, data: &ImageBuffer<Luma<u8>, Vec<u8>>) {
    for row in data.rows() {
        let input: Vec<u32> = row.map(|el| *el.0.first().unwrap() as u32).collect();

        let output = transform::transform_1d(input.as_slice(), &config.filter_params);

        for idx in 0..5 {
            println!("{} {}", input[idx], output[idx]);
        }
        println!();
    }
}
