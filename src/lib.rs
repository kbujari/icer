mod filters;
mod wavelet;

use crate::wavelet::wavelet_transform;
use std::path::Path;

pub use crate::filters::FilterParams;

#[derive(Debug, Default, Clone, Copy)]
pub struct Icer {
    pub filter_params: FilterParams,
}

impl Icer {
    pub fn new() -> Self {
        Icer {
            filter_params: FilterParams::A,
        }
    }

    pub fn compress(&self, path: impl AsRef<Path>) {
        let img = image::open(&path).unwrap().into_luma8();
        wavelet_transform(self, &img);
    }
}
