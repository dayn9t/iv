use std::cmp::max;
use image::RgbImage;
use opencv::imgproc::{ellipse, get_font_scale_from_height, get_text_size, LINE_8, polylines, put_text, rectangle};

use iv_core::geo::{PointF, PolygonF, PolygonI};
pub use iv_core::geo::{Size, ToAcRect};

pub use crate::image::color::Rgb;
use crate::image::ocv::{cv, cv_ac_point, cv_ac_rect, cv_color_bgr, cv_points, image_as_mat};

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
    let color = cv_color_bgr(color);
    let mut mat = image_as_mat(canvas);

    rectangle(&mut mat, rect, color, thickness, LINE_8, 0).unwrap();
}

/// 绘制椭圆
pub fn draw_ellipse(canvas: &mut RgbImage, rect: impl ToAcRect, color: Rgb, thickness: i32) {
    let size = image_size(canvas);
    let rect = cv_ac_rect(rect, size);
    let center = cv::rect_center(rect);
    let color = cv_color_bgr(color);
    let mut mat = image_as_mat(canvas);

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
    let color = cv_color_bgr(color);
    let mut mat = image_as_mat(canvas);
    let polygon: PolygonI = polygon.absolutized(size).unwrap();
    let points = cv_points(polygon.into());

    polylines(&mut mat, &points, true, color, thickness, LINE_8, 0).unwrap();
}

/// 显示文字
pub fn draw_text(canvas: &mut RgbImage, text: &str, left_bottom: PointF, color: Rgb, thickness: i32, scale: f64) {
    let size = image_size(canvas);
    let left_bottom = cv_ac_point(left_bottom, size);
    let color = cv_color_bgr(color);
    let mut mat = image_as_mat(canvas);
    put_text(&mut mat, text, left_bottom, 0, scale, color, thickness, LINE_8, false).unwrap();
}

/*
def draw_boxi(image: ImageNda, rect: Rect, color: Color, label: str = '', thickness: int = 0) -> None:
    """绘制带标签的矩形框(整数坐标)"""
    # TODO: 移除cv2调用
    bgr = image.data()
    tl = thickness or round(0.002 * (bgr.shape[0] + bgr.shape[1]) / 2) + 1
    c1, c2 = (rect.x, rect.y), (rect.x + rect.width, rect.y + rect.height)
    cv2.rectangle(bgr, c1, c2, color.bgr(), thickness=tl, lineType=cv2.LINE_AA)
    if label:
        tf = max(tl - 1, 1)  # font thickness
        t_size = cv2.getTextSize(label, 0, fontScale=tl / 3, thickness=tf)[0]
        # c2 = c1[0] + t_size[0], c1[1] - t_size[1] - 3
        c2 = c1[0] + t_size[0], c1[1] + t_size[1] + 3
        cv2.rectangle(bgr, c1, c2, color.bgr(), -1, cv2.LINE_AA)  # filled
        color = color.inverse()
        # cv2.putText(bgr, label, (c1[0], c1[1] - 2), 0, tl / 3, color, thickness=tf, lineType=cv2.LINE_AA)
        cv2.putText(bgr, label, (c1[0], c1[1] - 2 + t_size[1] + 3), 0, tl / 3, color.bgr(), thickness=tf,
                    lineType=cv2.LINE_AA)

*/
// 绘制带标签的矩形框(整数坐标)
pub fn draw_box(canvas: &mut RgbImage, rect: impl ToAcRect, label: &str, font_height: i32, color: Rgb, thickness: i32) {
    let size = image_size(canvas);
    let rect = cv_ac_rect(rect, size);
    let color = cv_color_bgr(color);
    let mut mat = image_as_mat(canvas);

    rectangle(&mut mat, rect, color, thickness, LINE_8, 0).unwrap();

    let font_face = 0;

    if !label.is_empty() {
        let line_space = font_height;
        let text_thickness = max(thickness / 2, 1);
        let font_scale = get_font_scale_from_height(font_face, font_height, text_thickness).unwrap();
        let mut base_line = 0;
        let size = get_text_size(label, font_face, font_scale, thickness, &mut base_line).unwrap();
        let mut left_bottom = cv::Point::new(rect.x + line_space, rect.y + line_space + size.height);

        for text in label.split('\n') {
            put_text(&mut mat, text, left_bottom, font_face, font_scale, color, text_thickness, LINE_8, false).unwrap();
            left_bottom.y += line_space + size.height;
        }
    }
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use opencv::core::{MatTrait, Scalar};
    use opencv::highgui;

    use iv_core::geo::{IPolygon, RectF};

    use crate::image::{GREEN, load_image, RED, YELLOW};

    use super::*;

    #[test]
    fn test_draw() {
        let p = PathBuf::from("/home/jiang/rs/iv/iv-mm/data/lena.jpg");
        let im = load_image(&p).unwrap();
        let mut canvas = im.to_rgb8();

        let mut mat = image_as_mat(&mut canvas);

        let window = "lena";
        highgui::named_window(window, highgui::WINDOW_AUTOSIZE).unwrap();
        highgui::imshow(window, &mat).unwrap();
        let _key = highgui::wait_key(0).unwrap();

        mat.set_scalar(Scalar::all(255.0)).unwrap();
        let mat1 = image_as_mat(&mut canvas);

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
