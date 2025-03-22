use iv_core::geo::Size;
use iv_mm::image::{IImage2D, ImageRgb, Rgb, WHITE, resize, show_rgb};
use opencv::{core::Point, highgui};
use std::path::Path;

/*
fn mouse_callback(event: i32, x: i32, y: i32, flags: i32, param: &mut dyn std::any::Any) {
    if let Some(win) = param.downcast_mut::<Rc<RefCell<ImageWin>>>() {
        win.borrow_mut().on_mouse(event, Point::new(x, y), flags);
    }
}*/

pub trait OnUiEvent {
    /// 鼠标移动事件
    fn on_mouse_move(&mut self, _pos: Point, _flags: i32) {}
    /// 鼠标左键按下事件
    fn on_left_button_down(&mut self, _pos: Point, _flags: i32) {}
    /// 鼠标右键按下事件
    fn on_right_button_down(&mut self, _pos: Point, _flags: i32) {}
    /// 绘制图像
    fn on_draw(&mut self, _canvas: &mut ImageRgb, _pos: Point) {}
    /// 按键响应，默认Esc键退出，返回值: 0-继续运行，非0-退出程序
    fn on_key(&mut self, key: i32) -> i32 {
        if key == 27 {
            return 1;
        }
        0
    }
    /// 空闲事件
    fn on_idle(&mut self) {}
}

pub struct ImageWin {
    title: String,
    size: Size,
    canvas: ImageRgb,
    background: ImageRgb,
    pos: Point,
    events: Box<dyn OnUiEvent>,
}

impl ImageWin {
    pub fn new(title: &str, size: Size, events: Box<dyn OnUiEvent>) -> Self {
        let win = Self {
            title: title.to_string(),
            size,
            canvas: ImageRgb::new(size, WHITE),
            background: ImageRgb::new(size, WHITE),
            pos: Point::new(0, 0),
            events,
        };
        highgui::named_window(title, highgui::WINDOW_AUTOSIZE).unwrap();
        //let on_mouse = Box::new(Self::mouse_callback);
        //highgui::set_mouse_callback(title, Some(on_mouse)).unwrap();
        return win;
    }
    /*
    // 鼠标回调函数
    fn mouse_callback(event: i32, x: i32, y: i32, flags: i32) {
        unsafe {
            if !param.is_null() {
                let win = &mut *(param as *mut ImageWin);
                win._on_mouse(event, Point::new(x, y), flags);
            }
        }
    }*/

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn resize(&mut self, size: Size) {
        if self.size != size {
            self.size = size;
            self.canvas = ImageRgb::new_black(size);
        }
    }

    pub fn background(&self) -> &ImageRgb {
        &self.background
    }

    pub fn set_pos(&mut self, p: Point) {
        highgui::move_window(&self.title, p.x, p.y).unwrap();
    }

    pub fn set_background(&mut self, image: &ImageRgb) {
        self.background = image.clone();
    }

    pub fn set_backcolor(&mut self, color: Rgb) {
        self.background.fill_color(color);
    }

    pub fn canvas(&self) -> &ImageRgb {
        &self.canvas
    }

    pub fn snapshot(&self, file: &Path) {
        self.canvas.save(file).unwrap();
        println!("Snapshot saved: {:?}", file);
    }

    pub fn refresh(&mut self) {
        resize(&self.background, &mut self.canvas).unwrap();
        self.events.on_draw(&mut self.canvas, self.pos);
        show_rgb(&self.canvas, &self.title, -1);
    }

    pub fn run(&mut self, interval: i32) -> i32 {
        let mut key = -1;
        loop {
            self.refresh();
            if key > 0 {
                let r = self.events.on_key(key);
                if r != 0 {
                    highgui::destroy_all_windows().unwrap();
                    return r;
                }
            } else {
                self.events.on_idle();
            }
            key = highgui::wait_key(interval).unwrap();
        }
    }

    fn _on_mouse(&mut self, event: i32, pos: Point, flags: i32) {
        self.pos = pos;
        match event {
            highgui::EVENT_MOUSEMOVE => self.events.on_mouse_move(pos, flags),
            highgui::EVENT_LBUTTONDOWN => self.events.on_left_button_down(pos, flags),
            highgui::EVENT_RBUTTONDOWN => self.events.on_right_button_down(pos, flags),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_load_image() {}
}
