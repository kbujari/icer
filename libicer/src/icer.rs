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

    pub fn compress(&self, img_path: &path::PathBuf) {
        let img = image::open(&img_path).unwrap().into_luma8();
        println!("{:#?}", img);
        img.save("out.jpg").unwrap();
    }
}
