use iv_core::geo::{IPolygon, PointF, PolygonF, RectF};
use iv_mm::image::*;

use iv_core::geo::SIZE_HD;
use iv_gui::PACKAGE_DIR;
use iv_gui::image_win::{ImageWin, OnUiEvent};
use opencv::core::Point;
use path_macro::path;

#[derive(Default, Clone, Copy)]
struct ImageViewer {
    _pos: Point,
}

impl OnUiEvent for ImageViewer {
    fn on_draw(&mut self, canvas: &mut ImageRgb, _pos: Point) {
        //canvas.fill_color(random_color16())

        let r = RectF::new(0.25, 1.0 / 3.0, 0.5, 1.0 / 3.0);
        draw_rect(canvas, r, PEN_YELLOW);
        draw_ellipse(canvas, r, PEN_RED);
        let r = RectF::new(0.2, 0.2, 0.6, 0.6);
        let poly = PolygonF::from(r.vertices());
        draw_polygon(canvas, &poly, PEN_GREEN);
        let p = PointF::new(0.2, 0.2);
        draw_text(canvas, "hello", p, PEN_YELLOW, 0.8);
        //draw_box(&mut canvas, r, "Hello\nIt's me", 16, BLUE, 2, false);
        draw_box(canvas, r, "Hello\nIt's me", 16, PEN_BLUE, true);
    }
}

fn draw() {
    let file = path!(PACKAGE_DIR / "../assets/images/lena.jpg");

    let background = ImageRgb::load(&file).unwrap();

    let viewer = ImageViewer::default();

    let mut win = ImageWin::new("image", SIZE_HD, Box::new(viewer));

    win.set_background(&background);

    win.run(200);
}

fn main() {
    draw()
}
