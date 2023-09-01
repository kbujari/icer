use image::{buffer::Pixels, Luma};

use crate::FilterParams;

pub fn transform_1d(data: Pixels<Luma<u8>>, filter: &FilterParams) {
    let stop = data.len() / 2 - 1;
    let odd = data.len() & 1 == 1;

    let lp_outputs = data.take(stop).map(|p| p.0.first().unwrap()).enumerate();

    let (a_neg, a_zero, a_pos, b) = filter.to_params();
}
