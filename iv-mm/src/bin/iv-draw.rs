use iv_core::geo::{IPolygon, PolygonF, RectF};
use iv_mm::image::ocv::image_as_mat;
use iv_mm::image::{draw_ellipse, draw_polygon, draw_rect, load_image, GREEN, RED, YELLOW};
use opencv::core::{MatTrait, Scalar};
use opencv::highgui;
use std::path::PathBuf;

fn main() {
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
