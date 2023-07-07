use crate::filter::FilterParams;

use std::path;

#[derive(Debug, Clone, Copy)]
pub struct Icer {
    pub filter_param: FilterParams,
}

impl Icer {
    pub fn new() -> Self {
        Icer {
            filter_param: FilterParams::A,
        }
    }

    pub fn compress(&self, path: &path::PathBuf) {
        println!("{:?}", path);
    }
}

impl Default for Icer {
    fn default() -> Self { Self::new() }
}
