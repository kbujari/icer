use image::{ImageBuffer, Luma};

pub fn transform_1d(data: &ImageBuffer<Luma<u8>, Vec<u8>>, n: usize) {
    let (width, height) = data.dimensions();
    let low_n = n / 2 - 1;
}
