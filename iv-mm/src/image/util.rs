use crate::image::ocv::{CvPoint, CvScalar, cv_points, gray_as_mut_mat};
use image::GrayImage;
use iv_core::geo::{PointFs, Points, Size, ToAcPoints};
use opencv::imgproc;

/// 根据多边形区域，生成掩码图像
pub fn make_mask(size: Size, roi: &PointFs, color: u8) -> GrayImage {
    let roi = roi.to_ac_points(size);
    make_mask_i32(size, &roi, color)
}

/// 根据多边形区域，生成掩码图像
pub fn make_mask_i32(size: Size, roi: &Points, color: u8) -> GrayImage {
    let mut mask = GrayImage::new(size.width as u32, size.height as u32);

    let mut mat = gray_as_mut_mat(&mut mask);

    let roi_mat = cv_points(roi.clone());
    imgproc::fill_poly(
        &mut mat,
        &roi_mat,
        CvScalar::all(color as f64),
        8,
        0,
        CvPoint::new(0, 0),
    )
    .unwrap();
    mask
}

#[cfg(test)]
mod tests {
    //use candle_core::Shape;
    use super::*;
    use crate::image::show_gray;
    use iv_core::geo::{Point, SIZE_NHD};

    #[test]
    fn test_mat() {
        let roi = vec![Point { x: 100, y: 100 }, Point { x: 200, y: 100 }, Point {
            x: 200,
            y: 200,
        }];

        let mask = make_mask_i32(SIZE_NHD, &roi, 255);
        show_gray(&mask, "mask", 0);
    }
}
