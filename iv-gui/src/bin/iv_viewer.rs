use iv_core::geo::SIZE_HD;
use iv_gui::image_win::{ImageWin, OnUiEvent};
use iv_mm::image::{BLACK, IImage2D, ImageRgb, random_color16};
use opencv::core::Point;

#[derive(Default, Clone, Copy)]
struct ImageViewer {
    _pos: Point,
}

impl OnUiEvent for ImageViewer {
    fn on_draw(&mut self, canvas: &mut ImageRgb, _pos: Point) {
        canvas.fill_color(random_color16())
    }
}

fn main() {
    let msg = ImageViewer::default();
    let mut win = ImageWin::new("image", SIZE_HD, Box::new(msg));
    win.run(10);
}
