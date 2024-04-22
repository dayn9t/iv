use std::ffi::c_void;

use image::RgbImage;
use opencv::core::CV_8UC3;
use opencv::prelude::*;

use iv_core::geo::{Point, Points, Rect, Size, ToAcPoint, ToAcRect};

use crate::image::Rgb;

pub mod cv {
    pub use opencv::core::Point;
    pub use opencv::core::Rect;
    pub use opencv::core::Scalar;
    pub use opencv::types::VectorOfPoint;

    /// 获取矩形中心
    pub fn rect_center(r: Rect) -> Point {
        Point::new(r.x + r.width / 2, r.y + r.height / 2)
    }
}

/// 点转化为CV点
pub fn cv_point(p: Point) -> cv::Point {
    cv::Point::new(p.x, p.y)
}

/// 点集转化为CV点集
pub fn cv_points(points: Points) -> cv::VectorOfPoint {
    cv::VectorOfPoint::from_iter(points.into_iter().map(|p| cv_point(p)))
}

/// 矩形转化为CV矩形
pub fn cv_rect(r: Rect) -> cv::Rect {
    cv::Rect::new(r.x, r.y, r.width, r.height)
}

/// 点(绝对/归一化坐标)转化为CV点(绝对坐标)
pub fn cv_ac_point(p: impl ToAcPoint, size: Size) -> cv::Point {
    let p = p.to_ac_point(size);
    cv_point(p)
}

/// 矩形(绝对/归一化坐标)转化为CV矩形(绝对坐标)
pub fn cv_ac_rect(r: impl ToAcRect, size: Size) -> cv::Rect {
    let r = r.to_ac_rect(size);
    cv_rect(r)
}

/// 颜色转换为CV类型
pub fn cv_color(rgb: Rgb) -> cv::Scalar {
    cv::Scalar::new(rgb.r() as f64, rgb.g() as f64, rgb.b() as f64, 0.0)
}

/// 颜色转换为CV类型, RGB=>BGR
pub fn cv_color_bgr(rgb: Rgb) -> cv::Scalar {
    cv::Scalar::new(rgb.b() as f64, rgb.g() as f64, rgb.r() as f64, 0.0)
}

/// Image(RGB) 作为 Mat(RGB)
pub fn image_as_mat(image: &mut RgbImage) -> Mat {
    let (width, height) = image.dimensions();
    unsafe {
        let p = image.as_mut_ptr() as *mut c_void;

        Mat::new_rows_cols_with_data_unsafe_def(height as i32, width as i32, CV_8UC3, p).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use opencv::core::Scalar;
    use opencv::highgui;

    use iv_core::geo::{PointF, RectF};

    use crate::image::load_image;

    use super::*;

    #[test]
    fn test_point() {
        let size = Size::new(10, 10);

        let pi = Point::new(1, 1);
        let pf = PointF::new(0.1, 0.1);
        let pcv = cv::Point::new(1, 1);

        assert_eq!(cv_ac_point(pi, size), pcv);
        assert_eq!(cv_ac_point(pf, size), pcv);
    }

    #[test]
    fn test_rect() {
        let size = Size::new(10, 10);

        let ri = Rect::new(1, 1, 1, 2);
        let rf = RectF::new(0.1, 0.1, 0.1, 0.2);
        let rcv = cv::Rect::new(1, 1, 1, 2);

        assert_eq!(cv_ac_rect(ri, size), rcv);
        assert_eq!(cv_ac_rect(rf, size), rcv);
    }

    #[test]
    fn test_mat() {
        let p = PathBuf::from("/home/jiang/rs/iv/iv-mm/data/lena.jpg");
        let im = load_image(&p).unwrap();
        let mut im = im.to_rgb8();

        let mut mat = image_as_mat(&mut im);

        let window = "lena";
        highgui::named_window(window, highgui::WINDOW_AUTOSIZE).unwrap();
        highgui::imshow(window, &mat).unwrap();
        let _key = highgui::wait_key(0).unwrap();

        mat.set_scalar(Scalar::all(255.0)).unwrap();
        let mat1 = image_as_mat(&mut im);
        highgui::imshow(window, &mat1).unwrap();
        let _key = highgui::wait_key(0).unwrap();
    }
}
