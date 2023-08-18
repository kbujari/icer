pub mod transform;

use crate::FilterParams;
use image::{ImageBuffer, Luma};

pub fn wavelet_transform(data: &ImageBuffer<Luma<u8>, Vec<u8>>, filter: &FilterParams) {
    let (width, height) = data.dimensions();
}
