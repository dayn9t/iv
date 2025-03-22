use crate::geo::RectT;
pub use geo_types::CoordNum;

pub trait IRect<T: CoordNum> {
    /// 返回矩形左上角x坐标
    fn x(&self) -> T;

    /// 返回矩形左上角y坐标
    fn y(&self) -> T;

    /// 返回矩形左上角宽度
    fn width(&self) -> T;

    /// 返回矩形左上角高度
    fn height(&self) -> T;

    /// 获取尺寸
    fn rect(&self) -> RectT<T> {
        RectT {
            x: self.x(),
            y: self.y(),
            width: self.width(),
            height: self.height(),
        }
    }
    /// 获取尺寸的面积
    fn area(&self) -> T {
        self.width() * self.height()
    }
}

impl<T: CoordNum> IRect<T> for RectT<T> {
    fn x(&self) -> T {
        self.x
    }

    fn y(&self) -> T {
        self.y
    }

    fn width(&self) -> T {
        self.width
    }

    fn height(&self) -> T {
        self.height
    }
}
