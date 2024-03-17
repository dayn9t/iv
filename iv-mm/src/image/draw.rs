use image::RgbImage;
use opencv::imgproc::{ellipse, polylines, rectangle, LINE_8};

use iv_core::geo::{PolygonF, PolygonI};
pub use iv_core::geo::{Size, ToAcRect};

pub use crate::image::color::Rgb;
use crate::image::ocv::{cv, cv_ac_rect, cv_color_bgr, cv_points, image_as_mat};

pub fn image_size(image: &RgbImage) -> Size {
    Size {
        width: image.width() as i32,
        height: image.height() as i32,
    }
}

/// 绘制矩形
pub fn draw_rect(image: &mut RgbImage, rect: impl ToAcRect, color: Rgb, thickness: i32) {
    let size = image_size(image);
    let rect = cv_ac_rect(rect, size);
    let color = cv_color_bgr(color);
    let mut mat = image_as_mat(image);

    rectangle(&mut mat, rect, color, thickness, LINE_8, 0).unwrap();
}

/// 绘制椭圆
pub fn draw_ellipse(image: &mut RgbImage, rect: impl ToAcRect, color: Rgb, thickness: i32) {
    let size = image_size(image);
    let rect = cv_ac_rect(rect, size);
    let center = cv::rect_center(rect);
    let color = cv_color_bgr(color);
    let mut mat = image_as_mat(image);

    ellipse(
        &mut mat,
        center,
        rect.size() / 2,
        0.0,
        0.0,
        360.0,
        color,
        thickness,
        LINE_8,
        0,
    )
    .unwrap();
}

/// 绘制多边形
pub fn draw_polygon(image: &mut RgbImage, polygon: &PolygonF, color: Rgb, thickness: i32) {
    let size = image_size(image);
    let color = cv_color_bgr(color);
    let mut mat = image_as_mat(image);
    let polygon: PolygonI = polygon.absolutized(size).unwrap();
    let points = cv_points(polygon.into());

    polylines(&mut mat, &points, true, color, thickness, LINE_8, 0).unwrap();
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use opencv::core::{MatTrait, Scalar};
    use opencv::highgui;

    use iv_core::geo::{IPolygon, RectF};

    use crate::image::{load_image, GREEN, RED, YELLOW};

    use super::*;

    #[test]
    fn test_draw() {
        let p = PathBuf::from("/home/jiang/rs/iv/iv-mm/data/lena.jpg");
        let im = load_image(&p).unwrap();
        let mut image = im.to_rgb8();

        let mut mat = image_as_mat(&mut image);

        let window = "lena";
        highgui::named_window(window, highgui::WINDOW_AUTOSIZE).unwrap();
        highgui::imshow(window, &mat).unwrap();
        let _key = highgui::wait_key(0).unwrap();

        mat.set_scalar(Scalar::all(255.0)).unwrap();
        let mat1 = image_as_mat(&mut image);

        let r = RectF::new(0.25, 1.0 / 3.0, 0.5, 1.0 / 3.0);
        draw_rect(&mut image, r, YELLOW, 3);
        draw_ellipse(&mut image, r, RED, 1);
        let r = RectF::new(0.2, 0.2, 0.6, 0.6);
        let poly = PolygonF::from(r.vertices());
        draw_polygon(&mut image, &poly, GREEN, 1);

        highgui::imshow(window, &mat1).unwrap();
        let _key = highgui::wait_key(0).unwrap();
    }
}
