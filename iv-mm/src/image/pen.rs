use std::fmt;
use std::fmt::Display;
use crate::image::{BLACK, BLUE, CYAN, GRAY, GREEN, ORANGE, PURPLE, RED, Rgb, WHITE, YELLOW};

/// 画笔
#[derive(Copy, Clone, Debug, Default, PartialEq)] //, Serialize, Deserialize
pub struct Pen(Rgb, i32);

impl Pen {
    pub fn new(color: Rgb, thickness:i32) -> Self {
        Self(color, thickness)
    }

    /// 获取画笔颜色
    pub fn color(&self) -> Rgb {
        self.0
    }

    /// 获取画笔线宽
    pub fn thickness(&self) -> i32 {
        self.1
    }

}

impl Display for Pen {
    ///格式化显示ID
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", self.0, self.1)
    }
}

/// 红笔
pub const RED_PEN: Pen = Pen(RED, 1);
/// 橙笔
pub const ORANGE_PEN: Pen = Pen(ORANGE, 1);
/// 黄笔
pub const YELLOW_PEN: Pen = Pen(YELLOW, 1);
/// 绿笔
pub const GREEN_PEN: Pen = Pen(GREEN, 1);
/// 青笔
pub const CYAN_PEN: Pen = Pen(CYAN, 1);
/// 蓝笔
pub const BLUE_PEN: Pen = Pen(BLUE, 1);
/// 紫笔
pub const PURPLE_PEN: Pen = Pen(PURPLE, 1);
/// 黑笔
pub const BLACK_PEN: Pen = Pen(BLACK, 1);
/// 白笔
pub const WHITE_PEN: Pen = Pen(WHITE, 1);
/// 灰笔
pub const GRAY_PEN: Pen = Pen(GRAY, 1);
