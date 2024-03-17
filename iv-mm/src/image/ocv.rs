use std::ffi::c_void;
use std::slice;

use image::RgbImage;
use opencv::core::{_InputArray, _OutputArray, CV_8UC3, Mat, ToInputArray, ToOutputArray};
use opencv::imgproc::cvt_color;
use opencv::prelude::*;

mod ip {
    pub type Rgb = image::Rgb<u8>;
}

/// Buffer(RGB) => Mat(BGR)
fn buffer_rgb_to_mat_bgr(data: &[u8], width: usize, height: usize) -> Mat {
    unsafe {
        let mut mat = Mat::new_rows_cols(height as i32, width as i32, CV_8UC3).unwrap();
        let mat_data = mat.data_mut();
        let mat_data_slice = slice::from_raw_parts_mut(mat_data, data.len());
        mat_data_slice.copy_from_slice(data);
        //cvt_color(&mat, &mut mat, opencv::imgproc::COLOR_RGB2BGR, 0).unwrap();
        mat
    }
}

pub fn image_rgb_to_mat_bgr(image: RgbImage) -> Mat {
    let (width, height) = image.dimensions();
    let buffer = image.into_raw();

    buffer_rgb_to_mat_bgr(&buffer, width as usize, height as usize)
}


/*
/// Point转换
fn ip_point(p: &Point) -> ip::Point<i32> {
    ip::Point { x: p.x, y: p.y }
}

/// Rect准换
fn ip_rect(r: &Rect) -> ip::Rect {
    ip::Rect::at(r.x, r.y).of_size(r.width as u32, r.height as u32)
}
*/


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use opencv::highgui;

    use crate::image::load_image;

    use super::*;

    #[test]
    fn it_works() {
        let p = PathBuf::from("/home/jiang/rs/iv/iv-mm/data/lena.jpg");
        let im = load_image(&p).unwrap();
        let im = im.to_rgb8();

        let mat = image_rgb_to_mat_bgr(im);

        let window = "video capture";
        highgui::named_window(window, highgui::WINDOW_AUTOSIZE).unwrap();
        highgui::imshow(window, &mat).unwrap();

        let key = highgui::wait_key(0).unwrap();
    }
}

