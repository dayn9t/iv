use std::fmt;
use std::fmt::Display;

/// 颜色定义
#[derive(Copy, Clone, Debug, Default, PartialEq)] //, Serialize, Deserialize
pub struct Rgb(u8, u8, u8);

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    /// 获取红色分量
    pub fn r(&self) -> u8 {
        self.0
    }

    /// 获取绿色分量
    pub fn g(&self) -> u8 {
        self.1
    }

    /// 获取蓝色分量
    pub fn b(&self) -> u8 {
        self.2
    }
    /// 获取 [R, G, B]
    pub fn rgb(&self) -> [u8; 3] {
        [self.0, self.1, self.2]
    }

    /// 获取 [B, G, R]
    pub fn bgr(&self) -> [u8; 3] {
        [self.2, self.1, self.0]
    }

    /// 获取反色
    pub fn inverse(&self) -> Self {
        Self(255 - self.0, 255 - self.1, 255 - self.2)
    }

    /// RGB转换为单个整数
    pub fn as_i32(&self) -> i32 {
        (self.0 as i32) << 16 | (self.1 as i32) << 8 | (self.2 as i32)
    }
}

impl Into<[u8; 3]> for Rgb {
    fn into(self) -> [u8; 3] {
        [self.0, self.1, self.2]
    }
}

impl Display for Rgb {
    ///格式化显示ID
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}

/*
    @classmethod
    def try_parse(cls, name: str) -> Optional[Self]:
        """从颜色名称或者#RGB建立颜色"""
        if name.startswith('#'):
            s = name.replace('#', '0x')
            i = int(s, 16)
            b = i % 256
            i = i // 256
            g = i % 256
            r = i // 256
            return Color(r, g, b)
        else:
            name = name.upper().replace(' ', '_')
            return colors.get(name, None)

    @classmethod
    def parse(cls, name: str) -> Self:
        """从颜色名称或者#RGB建立颜色"""
        c = cls.try_parse(name)
        assert c, f'Invalid color: {name}'
        return c
*/

/// 粉红
pub const PINK: Rgb = Rgb(255, 192, 203);
/// 猩红
pub const CRIMSON: Rgb = Rgb(220, 20, 60);
/// 淡紫红
pub const LAVENDER_BLUSH: Rgb = Rgb(255, 240, 245);
/// 苍白的紫罗兰红色
pub const PALE_VIOLET_RED: Rgb = Rgb(219, 112, 147);
/// 热粉红
pub const HOT_PINK: Rgb = Rgb(255, 105, 180);
/// 深粉红
pub const DEEP_PINK: Rgb = Rgb(255, 20, 147);
/// 适中的紫罗兰红色
pub const MEDIUM_VIOLET_RED: Rgb = Rgb(199, 21, 133);
/// 兰花紫
pub const ORCHID: Rgb = Rgb(218, 112, 214);
/// 蓟
pub const THISTLE: Rgb = Rgb(216, 191, 216);
/// 李子色
pub const PLUM: Rgb = Rgb(221, 160, 221);
/// 紫罗兰
pub const VIOLET: Rgb = Rgb(238, 130, 238);
/// 洋红
pub const MAGENTA: Rgb = Rgb(255, 0, 255);
/// 紫红色
pub const FUCHSIA: Rgb = Rgb(255, 0, 255);
/// 深洋红色
pub const DARK_MAGENTA: Rgb = Rgb(139, 0, 139);
/// 紫色
pub const PURPLE: Rgb = Rgb(128, 0, 128);
/// 适中的兰花紫
pub const MEDIUM_ORCHID: Rgb = Rgb(186, 85, 211);
/// 深紫罗兰色
pub const DARK_VIOLET: Rgb = Rgb(148, 0, 211);
/// 深兰花紫
pub const DARK_ORCHID: Rgb = Rgb(153, 50, 204);
/// 靛青
pub const INDIGO: Rgb = Rgb(75, 0, 130);
/// 紫罗兰的蓝色
pub const BLUE_VIOLET: Rgb = Rgb(138, 43, 226);
/// 适中的紫色
pub const MEDIUM_PURPLE: Rgb = Rgb(147, 112, 219);
/// 适中的板岩暗蓝灰色
pub const MEDIUM_SLATE_BLUE: Rgb = Rgb(123, 104, 238);
/// 板岩暗蓝灰色
pub const SLATE_BLUE: Rgb = Rgb(106, 90, 205);
/// 深板岩暗蓝灰色
pub const DARK_SLATE_BLUE: Rgb = Rgb(72, 61, 139);
/// 薰衣草花的淡紫色
pub const LAVENDER: Rgb = Rgb(230, 230, 250);
/// 幽灵的白色
pub const GHOST_WHITE: Rgb = Rgb(248, 248, 255);
/// 蓝
pub const BLUE: Rgb = Rgb(0, 0, 255);
/// 适中的蓝色
pub const MEDIUM_BLUE: Rgb = Rgb(0, 0, 205);
/// 午夜的蓝色
pub const MIDNIGHT_BLUE: Rgb = Rgb(25, 25, 112);
/// 深蓝色
pub const DARK_BLUE: Rgb = Rgb(0, 0, 139);
/// 海军蓝
pub const NAVY: Rgb = Rgb(0, 0, 128);
/// 皇家蓝
pub const ROYAL_BLUE: Rgb = Rgb(65, 105, 255);
/// 矢车菊的蓝色
pub const CORNFLOWER_BLUE: Rgb = Rgb(100, 149, 237);
/// 淡钢蓝
pub const LIGHT_STEEL_BLUE: Rgb = Rgb(176, 196, 222);
/// 浅石板灰
pub const LIGHT_SLATE_GRAY: Rgb = Rgb(119, 136, 153);
/// 石板灰
pub const SLATE_GRAY: Rgb = Rgb(112, 128, 144);
/// 道奇蓝
pub const DODGER_BLUE: Rgb = Rgb(30, 144, 255);
/// 爱丽丝蓝
pub const ALICE_BLUE: Rgb = Rgb(240, 248, 255);
/// 钢蓝
pub const STEEL_BLUE: Rgb = Rgb(70, 130, 180);
/// 淡天蓝色
pub const LIGHT_SKY_BLUE: Rgb = Rgb(135, 206, 250);
/// 天蓝色
pub const SKY_BLUE: Rgb = Rgb(135, 206, 235);
/// 深天蓝
pub const DEEP_SKY_BLUE: Rgb = Rgb(0, 191, 255);
/// 淡蓝色
pub const LIGHT_BLUE: Rgb = Rgb(173, 216, 230);
/// 浅灰蓝
pub const POWDER_BLUE: Rgb = Rgb(176, 224, 230);
/// 军校蓝
pub const CADET_BLUE: Rgb = Rgb(95, 158, 160);
/// 蔚蓝色
pub const AZURE: Rgb = Rgb(240, 255, 255);
/// 浅青色
pub const LIGHT_CYAN: Rgb = Rgb(224, 255, 255);
/// 苍白的绿宝石
pub const PALE_TURQUOISE: Rgb = Rgb(175, 238, 238);
/// 青色
pub const CYAN: Rgb = Rgb(0, 255, 255);
/// 水绿色
pub const AQUA: Rgb = Rgb(0, 255, 255);
/// 深绿宝石
pub const DARK_TURQUOISE: Rgb = Rgb(0, 206, 209);
/// 深石板灰
pub const DARK_SLATE_GRAY: Rgb = Rgb(47, 79, 79);
/// 深青色
pub const DARK_CYAN: Rgb = Rgb(0, 139, 139);
/// 水鸭色
pub const TEAL: Rgb = Rgb(0, 128, 128);
/// 适中的绿宝石
pub const MEDIUM_TURQUOISE: Rgb = Rgb(72, 209, 204);
/// 浅海洋绿
pub const LIGHT_SEA_GREEN: Rgb = Rgb(32, 178, 170);
/// 绿宝石
pub const TURQUOISE: Rgb = Rgb(64, 224, 208);
/// 绿玉，碧绿色
pub const AQUAMARINE: Rgb = Rgb(127, 255, 212);
/// 适中的碧绿色
pub const MEDIUM_AQUAMARINE: Rgb = Rgb(102, 205, 170);
/// 适中的春天的绿色
pub const MEDIUM_SPRING_GREEN: Rgb = Rgb(0, 250, 154);
/// 薄荷奶油
pub const MINT_CREAM: Rgb = Rgb(245, 255, 250);
/// 春天的绿色
pub const SPRING_GREEN: Rgb = Rgb(0, 255, 127);
/// 适中的海洋绿
pub const MEDIUM_SEA_GREEN: Rgb = Rgb(60, 179, 113);
/// 海洋绿
pub const SEA_GREEN: Rgb = Rgb(46, 139, 87);
/// 蜂蜜
pub const HONEYDEW: Rgb = Rgb(240, 255, 240);
/// 淡绿色
pub const LIGHT_GREEN: Rgb = Rgb(144, 238, 144);
/// 苍白的绿色
pub const PALE_GREEN: Rgb = Rgb(152, 251, 152);
/// 深海洋绿
pub const DARK_SEA_GREEN: Rgb = Rgb(143, 188, 143);
/// 酸橙绿
pub const LIME_GREEN: Rgb = Rgb(50, 205, 50);
/// 酸橙色
pub const LIME: Rgb = Rgb(0, 255, 0);
/// 森林绿
pub const FOREST_GREEN: Rgb = Rgb(34, 139, 34);
/// 绿
pub const GREEN: Rgb = Rgb(0, 128, 0);
/// 深绿色
pub const DARK_GREEN: Rgb = Rgb(1, 100, 0);
/// 查特酒绿，淡黄绿色
pub const CHARTREUSE: Rgb = Rgb(127, 255, 0);
/// 草坪绿
pub const LAWN_GREEN: Rgb = Rgb(124, 252, 0);
/// 绿黄色
pub const GREEN_YELLOW: Rgb = Rgb(173, 255, 47);
/// 深橄榄绿
pub const DARK_OLIVE_GREEN: Rgb = Rgb(85, 107, 47);
/// 黄绿
pub const YELLOW_GREEN: Rgb = Rgb(154, 205, 50);
/// 橄榄土褐色
pub const OLIVE_DRAB: Rgb = Rgb(107, 142, 35);
/// 米色，浅褐色
pub const BEIGE: Rgb = Rgb(245, 245, 220);
/// 浅秋麒麟黄
pub const LIGHT_GOLDENROD_YELLOW: Rgb = Rgb(250, 250, 210);
/// 象牙
pub const IVORY: Rgb = Rgb(255, 255, 240);
/// 浅黄色
pub const LIGHT_YELLOW: Rgb = Rgb(255, 255, 224);
/// 黄
pub const YELLOW: Rgb = Rgb(255, 255, 0);
/// 橄榄色
pub const OLIVE: Rgb = Rgb(128, 128, 0);
/// 深卡其布
pub const DARK_KHAKI: Rgb = Rgb(189, 183, 107);
/// 柠檬薄纱
pub const LEMON_CHIFFON: Rgb = Rgb(255, 250, 205);
/// 灰秋麒麟
pub const PALE_GOLDENROD: Rgb = Rgb(238, 232, 170);
/// 卡其布
pub const KHAKI: Rgb = Rgb(240, 230, 140);
/// 金色
pub const GOLD: Rgb = Rgb(255, 215, 0);
/// 玉米色
pub const CORNSILK: Rgb = Rgb(255, 248, 220);
/// 秋麒麟色
pub const GOLDENROD: Rgb = Rgb(218, 165, 32);
/// 深秋麒麟
pub const DARK_GOLDENROD: Rgb = Rgb(184, 134, 11);
/// 花卉白
pub const FLORAL_WHITE: Rgb = Rgb(255, 250, 240);
/// 老饰带
pub const OLDLACE: Rgb = Rgb(253, 245, 230);
/// 小麦色
pub const WHEAT: Rgb = Rgb(245, 222, 179);
/// 鹿皮鞋
pub const MOCCASIN: Rgb = Rgb(255, 228, 181);
/// 橙色
pub const ORANGE: Rgb = Rgb(255, 165, 0);
/// 番木瓜色
pub const PAPAYA_WHIP: Rgb = Rgb(255, 239, 213);
/// 漂白的杏仁
pub const BLANCHED_ALMOND: Rgb = Rgb(255, 235, 205);
/// 纳瓦霍白
pub const NAVAJO_WHITE: Rgb = Rgb(255, 222, 173);
/// 古典白
pub const ANTIQUE_WHITE: Rgb = Rgb(250, 235, 215);
/// 棕褐色，茶色
pub const TAN: Rgb = Rgb(210, 180, 140);
/// 原木色
pub const BURLYWOOD: Rgb = Rgb(222, 184, 135);
/// 浓汤色
pub const BISQUE: Rgb = Rgb(255, 228, 196);
/// 深橙黄
pub const DARK_ORANGE: Rgb = Rgb(255, 140, 0);
/// 亚麻布色
pub const LINEN: Rgb = Rgb(250, 240, 230);
/// 秘鲁色
pub const PERU: Rgb = Rgb(205, 133, 63);
/// 桃色
pub const PEACH_PUFF: Rgb = Rgb(255, 218, 185);
/// 沙棕色
pub const SANDY_BROWN: Rgb = Rgb(244, 164, 96);
/// 巧克力色
pub const CHOCOLATE: Rgb = Rgb(210, 105, 30);
/// 马鞍棕色
pub const SADDLE_BROWN: Rgb = Rgb(139, 69, 19);
/// 海贝色
pub const SEA_SHELL: Rgb = Rgb(255, 245, 238);
/// 赭色
pub const SIENNA: Rgb = Rgb(160, 82, 45);
/// 浅肉色
pub const LIGHT_SALMON: Rgb = Rgb(255, 160, 122);
/// 珊瑚色
pub const CORAL: Rgb = Rgb(255, 127, 80);
/// 橙红色
pub const ORANGE_RED: Rgb = Rgb(255, 69, 0);
/// 深肉色
pub const DARK_SALMON: Rgb = Rgb(233, 150, 122);
/// 番茄色
pub const TOMATO: Rgb = Rgb(255, 99, 71);
/// 薄雾玫瑰色
pub const MISTY_ROSE: Rgb = Rgb(255, 228, 225);
/// 鲜肉色
pub const SALMON: Rgb = Rgb(250, 128, 114);
/// 雪白
pub const SNOW: Rgb = Rgb(255, 250, 250);
/// 浅珊瑚色
pub const LIGHT_CORAL: Rgb = Rgb(240, 128, 128);
/// 玫瑰棕色
pub const ROSY_BROWN: Rgb = Rgb(188, 143, 143);
/// 印度红
pub const INDIAN_RED: Rgb = Rgb(205, 92, 92);
/// 红
pub const RED: Rgb = Rgb(255, 0, 0);
/// 褐色
pub const BROWN: Rgb = Rgb(165, 42, 42);
/// 耐火砖色
pub const FIRE_BRICK: Rgb = Rgb(178, 34, 34);
/// 深红
pub const DARK_RED: Rgb = Rgb(139, 0, 0);
/// 栗色
pub const MAROON: Rgb = Rgb(128, 0, 0);
/// 白
pub const WHITE: Rgb = Rgb(255, 255, 255);
/// 白烟
pub const WHITE_SMOKE: Rgb = Rgb(245, 245, 245);
/// 淡灰色
pub const GAINSBORO: Rgb = Rgb(220, 220, 220);
/// 浅灰
pub const LIGHT_GREY: Rgb = Rgb(211, 211, 211);
/// 银白
pub const SILVER: Rgb = Rgb(192, 192, 192);
/// 深灰
pub const DARK_GRAY: Rgb = Rgb(169, 169, 169);
/// 灰色
pub const GRAY: Rgb = Rgb(128, 128, 128);
/// 暗灰色
pub const DIM_GRAY: Rgb = Rgb(105, 105, 105);
/// 黑
pub const BLACK: Rgb = Rgb(0, 0, 0);
/// YOLO灰色
pub const YOLO_GRAY: Rgb = Rgb(114, 114, 114); //  # /opt/ias/env/lib/yolo5/utils/general.py;

/// 七色
pub const COLORS7: [Rgb; 7] = [RED, ORANGE, YELLOW, GREEN, CYAN, BLUE, PURPLE];

/// W3C16色
pub const W3C16: [Rgb; 16] = [
    BLACK, GREEN, SILVER, LIME, GRAY, OLIVE, WHITE, YELLOW, MAROON, NAVY, RED, BLUE, PURPLE, TEAL,
    FUCHSIA, AQUA,
];

use rand::Rng;

/// 从W3C16随机取出一种颜色
pub fn random_color16() -> Rgb {
    let idx = rand::rng().random_range(0..W3C16.len());
    W3C16[idx]
}

/// 从COLORS7随机取出一种颜色
pub fn random_color7() -> Rgb {
    let idx = rand::rng().random_range(0..COLORS7.len());
    COLORS7[idx]
}
