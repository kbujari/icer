pub mod icer;
pub mod params;

use std::path;

pub fn def_img(img_path: &path::Path) {
    let img = image::open(img_path).unwrap();
    img.into_luma8().save("black.jpg").unwrap();
}
