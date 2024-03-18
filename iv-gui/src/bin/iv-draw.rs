use iv_core::geo::{IPolygon, PointF, PolygonF, RectF};
use iv_mm::image::ocv::image_as_mat;
use iv_mm::image::{draw_ellipse, draw_polygon, draw_rect, load_image, GREEN, RED, YELLOW, draw_text, draw_box, BLUE};
use opencv::core::{MatTrait, Scalar};
use opencv::highgui;
use std::path::PathBuf;

fn main() {
    let p = PathBuf::from("/home/jiang/rs/iv/iv-mm/data/lena.jpg");
    let im = load_image(&p).unwrap();
    let mut canvas = im.to_rgb8();

    let mut mat = image_as_mat(&mut canvas);

    let window = "lena";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE).unwrap();
    highgui::imshow(window, &mat).unwrap();
    let _key = highgui::wait_key(0).unwrap();

    mat.set_scalar(Scalar::all(127.0)).unwrap();
    let mat1 = image_as_mat(&mut canvas);

    let r = RectF::new(0.25, 1.0 / 3.0, 0.5, 1.0 / 3.0);
    draw_rect(&mut canvas, r, YELLOW, 3);
    draw_ellipse(&mut canvas, r, RED, 1);
    let r = RectF::new(0.2, 0.2, 0.6, 0.6);
    let poly = PolygonF::from(r.vertices());
    draw_polygon(&mut canvas, &poly, GREEN, 1);
    let p = PointF::new(0.2, 0.2);
    draw_text(&mut canvas, "hello", p, YELLOW, 1, 0.8);
    draw_box(&mut canvas, r, "Hello\nIt's me\nhaha", 16, BLUE, 2);

    highgui::imshow(window, &mat1).unwrap();
    let _key = highgui::wait_key(0).unwrap();
}
