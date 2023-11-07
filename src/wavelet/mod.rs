mod transform;

use crate::{wavelet::transform::transform_1d, Icer};
use image::{ImageBuffer, Luma};

pub fn wavelet_transform(config: &Icer, data: &ImageBuffer<Luma<u8>, Vec<u8>>) {
    let mut out: Vec<Vec<u32>> = Vec::new();
    for row in data.rows() {
        let input: Vec<u8> = row.map(|el| *el.0.first().unwrap()).collect();
        out.push(transform_1d(input.as_slice(), &config.filter_params));
    }

    println!("{}", out.len());
}
