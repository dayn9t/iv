use std::ffi::c_void;

use image::{GrayImage, RgbImage};
use opencv::core::{CV_8UC1, CV_8UC2, CV_8UC3};
use opencv::imgproc::{COLOR_YUV2BGR_YUYV, COLOR_YUV2RGB_YUYV, cvt_color};
use opencv::prelude::*;

use iv_core::geo::{Point, Points, Rect, Size, ToAcPoint, ToAcRect};

use crate::image::Rgb;

pub type CvPoint = opencv::core::Point;
pub type CvSize = opencv::core::Size;
pub type CvRect = opencv::core::Rect;
pub type CvScalar = opencv::core::Scalar;
pub type CvPoints = opencv::core::Vector<CvPoint>;

/// 获取矩形中心
pub fn cv_rect_center(r: CvRect) -> CvPoint {
    CvPoint {
        x: r.x + r.width / 2,
        y: r.y + r.height / 2,
    }
}

/// 点转化为CV点
pub fn cv_point(p: Point) -> CvPoint {
    CvPoint::new(p.x, p.y)
}

/// 点集转化为CV点集
pub fn cv_points(points: Points) -> CvPoints {
    CvPoints::from_iter(points.into_iter().map(|p| cv_point(p)))
}

/// 矩形转化为CV矩形
pub fn cv_rect(r: Rect) -> CvRect {
    CvRect::new(r.x, r.y, r.width, r.height)
}

/// 点(绝对/归一化坐标)转化为CV点(绝对坐标)
pub fn cv_ac_point(p: impl ToAcPoint, size: Size) -> CvPoint {
    let p = p.to_ac_point(size);
    cv_point(p)
}

/// 矩形(绝对/归一化坐标)转化为CV矩形(绝对坐标)
pub fn cv_ac_rect(r: impl ToAcRect, size: Size) -> CvRect {
    let r = r.to_ac_rect(size);
    cv_rect(r)
}

/// 颜色转换为CV类型
pub fn cv_color(rgb: Rgb) -> CvScalar {
    CvScalar::new(rgb.r() as f64, rgb.g() as f64, rgb.b() as f64, 0.0)
}

/// 颜色转换为CV类型, RGB=>BGR
pub fn cv_color_bgr(rgb: Rgb) -> CvScalar {
    CvScalar::new(rgb.b() as f64, rgb.g() as f64, rgb.r() as f64, 0.0)
}

/// Image(RGB) 作为 Mat(RGB)
pub fn image_as_mut_mat(image: &mut RgbImage) -> Mat {
    let (width, height) = image.dimensions();
    unsafe {
        let p = image.as_mut_ptr() as *mut c_void;

        Mat::new_rows_cols_with_data_unsafe_def(height as i32, width as i32, CV_8UC3, p).unwrap()
    }
}

/// Image(RGB) 作为 Mat(RGB)
pub fn image_as_mat(image: &RgbImage) -> Mat {
    let (width, height) = image.dimensions();
    unsafe {
        let p = image.as_ptr() as *mut c_void;

        Mat::new_rows_cols_with_data_unsafe_def(height as i32, width as i32, CV_8UC3, p).unwrap()
    }
}

/// GrayImage 作为 Mat
pub fn gray_as_mat(image: &GrayImage) -> Mat {
    let (width, height) = image.dimensions();
    unsafe {
        let p = image.as_ptr() as *mut c_void;

        Mat::new_rows_cols_with_data_unsafe_def(height as i32, width as i32, CV_8UC1, p).unwrap()
    }
}

/// GrayImage 作为可修改 Mat
pub fn gray_as_mut_mat(image: &mut GrayImage) -> Mat {
    let (width, height) = image.dimensions();
    unsafe {
        let p = image.as_mut_ptr() as *mut c_void;

        Mat::new_rows_cols_with_data_unsafe_def(height as i32, width as i32, CV_8UC1, p).unwrap()
    }
}

/// 缓冲区抓换为Mat
pub fn buffer_as_mat(buffer: &[u8], size: Size, cv_type: i32) -> Mat {
    unsafe {
        let data_ptr = buffer.as_ptr() as *mut c_void;
        Mat::new_rows_cols_with_data_unsafe_def(size.height, size.width, cv_type, data_ptr).unwrap()
    }
}

/// YUYV422 转 Mat
pub fn yuyv_as_mat2c(buffer: &[u8], size: Size) -> Mat {
    buffer_as_mat(buffer, size, CV_8UC2)
}

/// YUYV422 转 RGB Mat
pub fn yuyv_to_bgr(buffer: &[u8], size: Size) -> Mat {
    let mat2c = yuyv_as_mat2c(buffer, size);
    unsafe {
        let mut mat3c = Mat::new_size(mat2c.size().unwrap(), CV_8UC3).unwrap();
        cvt_color(&mat2c, &mut mat3c, COLOR_YUV2BGR_YUYV, 0).unwrap();
        mat3c
    }
}

/// YUYV422 转 RGB Mat
pub fn yuyv_to_rgb1(buffer: &[u8], size: Size, mat3c: &mut Mat) {
    let mat2c = yuyv_as_mat2c(buffer, size);

    cvt_color(&mat2c, mat3c, COLOR_YUV2RGB_YUYV, 0).unwrap();
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use opencv::highgui;

    use iv_core::geo::{PointF, RectF};

    use crate::image::load_image;

    use super::*;

    #[test]
    fn test_point() {
        let size = Size::new(10, 10);

        let pi = Point::new(1, 1);
        let pf = PointF::new(0.1, 0.1);
        let pcv = CvPoint::new(1, 1);

        assert_eq!(cv_ac_point(pi, size), pcv);
        assert_eq!(cv_ac_point(pf, size), pcv);
    }

    #[test]
    fn test_rect() {
        let size = Size::new(10, 10);

        let ri = Rect::new(1, 1, 1, 2);
        let rf = RectF::new(0.1, 0.1, 0.1, 0.2);
        let rcv = CvRect::new(1, 1, 1, 2);

        assert_eq!(cv_ac_rect(ri, size), rcv);
        assert_eq!(cv_ac_rect(rf, size), rcv);
    }

    #[test]
    fn test_mat() {
        let p = PathBuf::from("/home/jiang/rs/iv/assets/images/lena.jpg");
        let im = load_image(&p).unwrap();
        let mut im = im.to_rgb8();

        let mut mat = image_as_mut_mat(&mut im);

        let window = "lena";
        highgui::named_window(window, highgui::WINDOW_AUTOSIZE).unwrap();
        highgui::imshow(window, &mat).unwrap();
        let _key = highgui::wait_key(0).unwrap();

        mat.set_scalar(CvScalar::all(255.0)).unwrap();
        let mat1 = image_as_mut_mat(&mut im);
        highgui::imshow(window, &mat1).unwrap();
        let _key = highgui::wait_key(0).unwrap();
    }
}
