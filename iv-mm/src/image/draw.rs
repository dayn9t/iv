use std::cmp::max;

use opencv::imgproc::{
    LINE_8, ellipse, get_font_scale_from_height, polylines, put_text, rectangle,
};

pub use crate::image::color::Rgb;
use crate::image::ocv::{
    CvPoint, cv_ac_point, cv_ac_rect, cv_color, cv_points, cv_rect_center, image_as_mut_mat,
};
use crate::image::{ImageRgb, WHITE};
use iv_core::geo::iface::ISize2D;
use iv_core::geo::{PointF, PolygonF, PolygonI};
pub use iv_core::geo::{Size, ToAcRect};

/// 绘制矩形
pub fn draw_rect(canvas: &mut ImageRgb, rect: impl ToAcRect, color: Rgb, thickness: i32) {
    let size = canvas.size();
    let rect = cv_ac_rect(rect, size);
    let color = cv_color(color);
    let mut mat = image_as_mut_mat(canvas);

    rectangle(&mut mat, rect, color, thickness, LINE_8, 0).unwrap();
}

/// 绘制椭圆
pub fn draw_ellipse(canvas: &mut ImageRgb, rect: impl ToAcRect, color: Rgb, thickness: i32) {
    let size = canvas.size();
    let rect = cv_ac_rect(rect, size);
    let center = cv_rect_center(rect);
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
pub fn draw_polygon(canvas: &mut ImageRgb, polygon: &PolygonF, color: Rgb, thickness: i32) {
    let size = canvas.size();
    let color = cv_color(color);
    let mut mat = image_as_mut_mat(canvas);
    let polygon: PolygonI = polygon.absolutized(size).unwrap();
    let points = cv_points(polygon.into());

    polylines(&mut mat, &points, true, color, thickness, LINE_8, 0).unwrap();
}

/// 显示文字
pub fn draw_text(
    canvas: &mut ImageRgb,
    text: &str,
    left_bottom: PointF,
    color: Rgb,
    thickness: i32,
    scale: f64,
) {
    let size = canvas.size();
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
    canvas: &mut ImageRgb,
    rect: impl ToAcRect,
    label: &str,
    font_height: i32,
    color: Rgb,
    thickness: i32,
    down_to_up: bool,
) {
    let size = canvas.size();
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
                CvPoint::new(rect.x + line_space, rect.y + rect.height - line_space),
                -dy,
            )
        } else {
            (CvPoint::new(rect.x + line_space, rect.y + dy), dy)
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
    #[test]
    fn test_mat() {}
}
