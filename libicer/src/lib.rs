mod filters;
mod wavelet;

use std::path::PathBuf;

pub use crate::filters::FilterParams;

#[derive(Debug, Clone, Copy)]
pub struct Icer {
    pub filter_params: FilterParams,
}

impl Icer {
    pub fn new() -> Self {
        Icer {
            filter_params: FilterParams::A,
        }
    }

    pub fn compress(&self, path: &PathBuf) {
        let img = image::open(path).unwrap().into_luma8();
        wavelet::wavelet_transform(&img, &self.filter_params);
    }
}

impl Default for Icer {
    fn default() -> Self {
        Self::new()
    }
}
