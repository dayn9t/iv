use crate::image::{BLACK, BLUE, CYAN, GRAY, GREEN, ORANGE, PURPLE, RED, Rgb, WHITE, YELLOW};
use std::fmt;
use std::fmt::Display;

/// 画笔
#[derive(Copy, Clone, Debug, Default, PartialEq)] //, Serialize, Deserialize
pub struct Pen {
    /// 画笔颜色
    pub color: Rgb,
    /// 获取画笔线宽
    pub thickness: i32,
}

impl Pen {
    pub fn new(color: Rgb, thickness: i32) -> Self {
        Self { color, thickness }
    }
}

impl Display for Pen {
    ///格式化显示ID
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", self.color, self.thickness)
    }
}

/// 红笔
pub const PEN_RED: Pen = Pen {
    color: RED,
    thickness: 2,
};
/// 橙笔
pub const PEN_ORANGE: Pen = Pen {
    color: ORANGE,
    thickness: 2,
};
/// 黄笔
pub const PEN_YELLOW: Pen = Pen {
    color: YELLOW,
    thickness: 2,
};
/// 绿笔
pub const PEN_GREEN: Pen = Pen {
    color: GREEN,
    thickness: 2,
};
/// 青笔
pub const PEN_CYAN: Pen = Pen {
    color: CYAN,
    thickness: 2,
};
/// 蓝笔
pub const PEN_BLUE: Pen = Pen {
    color: BLUE,
    thickness: 2,
};
/// 紫笔
pub const PEN_PURPLE: Pen = Pen {
    color: PURPLE,
    thickness: 2,
};
/// 黑笔
pub const PEN_BLACK: Pen = Pen {
    color: BLACK,
    thickness: 2,
};
/// 白笔
pub const PEN_WHITE: Pen = Pen {
    color: WHITE,
    thickness: 2,
};
/// 灰笔
pub const PEN_GRAY: Pen = Pen {
    color: GRAY,
    thickness: 2,
};
