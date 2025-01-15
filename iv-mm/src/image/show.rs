use image::{DynamicImage, GrayImage, RgbImage};
use opencv::highgui::{imshow, wait_key};
use opencv::imgproc::{COLOR_RGB2BGR, cvt_color};
use opencv::prelude::Mat;

use crate::image::ocv::{gray_as_mat, image_as_mat};

pub fn show(image: &DynamicImage, title: &str, delay: i32) {
    match image {
        DynamicImage::ImageLuma8(gray) => show_gray(&gray, title, delay),
        DynamicImage::ImageRgb8(rgb) => show_rgb(&rgb, title, delay),
        _ => unreachable!(),
    }
}

pub fn show_rgb(im: &RgbImage, title: &str, delay: i32) {
    let src = image_as_mat(im);
    let mut dst = Mat::default();
    cvt_color(&src, &mut dst, COLOR_RGB2BGR, 0).unwrap();
    imshow(title, &dst).unwrap();
    if delay >= 0 {
        wait_key(delay).unwrap();
    }
}

pub fn show_gray(im: &GrayImage, title: &str, delay: i32) {
    let mat = gray_as_mat(im);
    imshow(title, &mat).unwrap();
    if delay >= 0 {
        wait_key(delay).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use opencv::core::Scalar;
    use opencv::highgui;
    use opencv::prelude::MatTrait;

    use crate::image::load_image;
    use crate::image::ocv::image_as_mut_mat;

    #[test]
    fn test_mat() {
        let p = PathBuf::from("/home/jiang/rs/iv/iv-mm/assets/lena.jpg");
        let im = load_image(&p).unwrap();
        let mut im = im.to_rgb8();

        let mut mat = image_as_mut_mat(&mut im);

        let window = "lena";
        highgui::named_window(window, highgui::WINDOW_AUTOSIZE).unwrap();
        highgui::imshow(window, &mat).unwrap();
        let _key = highgui::wait_key(0).unwrap();

        mat.set_scalar(Scalar::all(255.0)).unwrap();
        let mat1 = image_as_mut_mat(&mut im);
        highgui::imshow(window, &mat1).unwrap();
        let _key = highgui::wait_key(0).unwrap();
    }
}
