use std::cmp::max;

use image::RgbImage;
use opencv::imgproc::{
    LINE_8, ellipse, get_font_scale_from_height, polylines, put_text, rectangle,
};

use iv_core::geo::{PointF, PolygonF, PolygonI};
pub use iv_core::geo::{Size, ToAcRect};

use crate::image::WHITE;
pub use crate::image::color::Rgb;
use crate::image::ocv::{cv, cv_ac_point, cv_ac_rect, cv_color, cv_points, image_as_mut_mat};

pub fn image_size(image: &RgbImage) -> Size {
    Size {
        width: image.width() as i32,
        height: image.height() as i32,
    }
}

/// 绘制矩形
pub fn draw_rect(canvas: &mut RgbImage, rect: impl ToAcRect, color: Rgb, thickness: i32) {
    let size = image_size(canvas);
    let rect = cv_ac_rect(rect, size);
    let color = cv_color(color);
    let mut mat = image_as_mut_mat(canvas);

    rectangle(&mut mat, rect, color, thickness, LINE_8, 0).unwrap();
}

/// 绘制椭圆
pub fn draw_ellipse(canvas: &mut RgbImage, rect: impl ToAcRect, color: Rgb, thickness: i32) {
    let size = image_size(canvas);
    let rect = cv_ac_rect(rect, size);
    let center = cv::rect_center(rect);
    let color = cv_color(color);
    let mut mat = image_as_mut_mat(canvas);

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
pub fn draw_polygon(canvas: &mut RgbImage, polygon: &PolygonF, color: Rgb, thickness: i32) {
    let size = image_size(canvas);
    let color = cv_color(color);
    let mut mat = image_as_mut_mat(canvas);
    let polygon: PolygonI = polygon.absolutized(size).unwrap();
    let points = cv_points(polygon.into());

    polylines(&mut mat, &points, true, color, thickness, LINE_8, 0).unwrap();
}

/// 显示文字
pub fn draw_text(
    canvas: &mut RgbImage,
    text: &str,
    left_bottom: PointF,
    color: Rgb,
    thickness: i32,
    scale: f64,
) {
    let size = image_size(canvas);
    let left_bottom = cv_ac_point(left_bottom, size);
    let color = cv_color(color);
    let mut mat = image_as_mut_mat(canvas);
    put_text(
        &mut mat,
        text,
        left_bottom,
        0,
        scale,
        color,
        thickness,
        LINE_8,
        false,
    )
    .unwrap();
}

// 绘制带标签的矩形框(整数坐标)
pub fn draw_box(
    canvas: &mut RgbImage,
    rect: impl ToAcRect,
    label: &str,
    font_height: i32,
    color: Rgb,
    thickness: i32,
    down_to_up: bool,
) {
    let size = image_size(canvas);
    let rect = cv_ac_rect(rect, size);
    let color = cv_color(color);
    let mut mat = image_as_mut_mat(canvas);

    rectangle(&mut mat, rect, color, thickness, LINE_8, 0).unwrap();

    let font_face = 0;

    if !label.is_empty() {
        let line_space = font_height / 2;
        let text_thickness = max(thickness / 2, 1);
        let font_scale =
            get_font_scale_from_height(font_face, font_height, text_thickness).unwrap();
        let dy = line_space + font_height;
        let (mut start_pos, dy) = if down_to_up {
            (
                cv::Point::new(rect.x + line_space, rect.y + rect.height - line_space),
                -dy,
            )
        } else {
            (cv::Point::new(rect.x + line_space, rect.y + dy), dy)
        };

        for text in label.split('\n') {
            put_text(
                &mut mat,
                text,
                start_pos,
                font_face,
                font_scale,
                cv_color(WHITE),
                text_thickness,
                LINE_8,
                false,
            )
            .unwrap();
            start_pos.y += dy;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use opencv::core::{MatTrait, Scalar};
    use opencv::highgui;

    use iv_core::geo::{IPolygon, RectF};

    use crate::image::{GREEN, RED, YELLOW, load_image};

    use super::*;

    #[test]
    fn test_draw() {
        let p = PathBuf::from("/home/jiang/rs/iv/iv-mm/data/lena.jpg");
        let im = load_image(&p).unwrap();
        let mut canvas = im.to_rgb8();

        let mut mat = image_as_mut_mat(&mut canvas);

        let window = "lena";
        highgui::named_window(window, highgui::WINDOW_AUTOSIZE).unwrap();
        highgui::imshow(window, &mat).unwrap();
        let _key = highgui::wait_key(0).unwrap();

        mat.set_scalar(Scalar::all(255.0)).unwrap();
        let mat1 = image_as_mut_mat(&mut canvas);

        let r = RectF::new(0.25, 1.0 / 3.0, 0.5, 1.0 / 3.0);
        draw_rect(&mut canvas, r, YELLOW, 3);
        draw_ellipse(&mut canvas, r, RED, 1);
        let r = RectF::new(0.2, 0.2, 0.6, 0.6);
        let poly = PolygonF::from(r.vertices());
        draw_polygon(&mut canvas, &poly, GREEN, 1);

        highgui::imshow(window, &mat1).unwrap();
        let _key = highgui::wait_key(0).unwrap();
    }
}
