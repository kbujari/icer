mod filters;
mod wavelet;

pub use filters::FilterParams;

pub fn compress(buffer: &mut [u16], input: &[u16], width: usize, filter: FilterParams) {
    wavelet::wavelet_transform(buffer, input, width, filter);
}

#[cfg(feature = "alloc")]
pub fn compress_file(path: impl AsRef<std::path::Path>, filter: FilterParams) -> Vec<u16> {
    let input = image::open(&path)
        .expect("provide existing image file")
        .into_luma16();

    let width = input.dimensions().0 as usize;

    let mut buf = vec![0u16; input.len()];

    self::compress(&mut buf, &input, width, filter);

    buf
}
