mod filters;
mod wavelet;

pub use filters::FilterParams;

pub fn compress(
    mut buffer: impl AsMut<[u16]>,
    input: impl AsRef<[u16]>,
    width: usize,
    filter: FilterParams,
) {
    wavelet::wavelet_transform(buffer.as_mut(), input.as_ref(), width, filter);
}

#[cfg(feature = "alloc")]
pub fn compress_file(path: impl AsRef<std::path::Path>, filter: FilterParams) -> Vec<u16> {
    let input = image::open(&path)
        .expect("provide existing image file")
        .into_luma16();

    let width = input.dimensions().0 as usize;
    let mut buf = vec![0u16; input.len()];

    self::compress(&mut buf, input.as_raw(), width, filter);

    buf
}
