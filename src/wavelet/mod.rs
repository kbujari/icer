mod transform;

use self::transform::transform_1d;
use crate::Icer;
use image::{ImageBuffer, Luma};

pub fn wavelet_transform(config: &Icer, data: &ImageBuffer<Luma<u8>, Vec<u8>>) {
    let mut out: Vec<Vec<u16>> = Vec::new();

    for row in data.rows() {
        let input: Vec<u16> = row.map(|el| el.0[0] as u16).collect();
        out.push(transform_1d(&input, &config.filter_params));
    }

    println!("{}", out.len());
}
