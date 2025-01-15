use crate::geo::{Point, PointF, PointFs, Points, Rect, RectF, Size};

/// 转换为归一化坐标
pub trait ToNcPoint {
    /// 转换为归一化坐标
    fn to_nc_point(&self, size: Size) -> PointF;
}

impl ToNcPoint for Point {
    /// 转换为归一化坐标, 整数坐标被认为是归一化坐标, 不需要转换
    fn to_nc_point(&self, size: Size) -> PointF {
        self.normalized(size).unwrap()
    }
}

impl ToNcPoint for PointF {
    /// 转换为归一化坐标, 整数坐标被认为是归一化坐标, 需要转换
    fn to_nc_point(&self, _size: Size) -> PointF {
        *self
    }
}

/// 转换为归一化坐标矩形
pub trait ToNcRect {
    /// 转换为归一化坐标矩形
    fn to_nc_rect(&self, size: Size) -> RectF;
}

impl ToNcRect for Rect {
    /// 转换为归一化坐标, 整数坐标被认为是归一化坐标, 不需要转换
    fn to_nc_rect(&self, size: Size) -> RectF {
        self.normalized(size).unwrap()
    }
}

impl ToNcRect for RectF {
    /// 转换为归一化坐标, 整数坐标被认为是归一化坐标, 需要转换
    fn to_nc_rect(&self, _size: Size) -> RectF {
        *self
    }
}

/// 转换为绝对坐标点
pub trait ToAcPoint {
    /// 转换为绝对坐标点
    fn to_ac_point(&self, size: Size) -> Point;
}

impl ToAcPoint for Point {
    /// 转换为绝对坐标, 整数坐标被认为是绝对坐标, 不需要转换
    fn to_ac_point(&self, _size: Size) -> Point {
        *self
    }
}

impl ToAcPoint for PointF {
    /// 转换为绝对坐标, 整数坐标被认为是归一化坐标, 需要转换
    fn to_ac_point(&self, size: Size) -> Point {
        self.absolutized(size).unwrap()
    }
}

/// 转换为绝对坐标矩形
pub trait ToAcRect {
    /// 转换为绝对坐标矩形
    fn to_ac_rect(&self, size: Size) -> Rect;
}

impl ToAcRect for Rect {
    /// 转换为绝对坐标, 整数坐标被认为是绝对坐标, 不需要转换
    fn to_ac_rect(&self, _size: Size) -> Rect {
        *self
    }
}

impl ToAcRect for RectF {
    /// 转换为绝对坐标, 整数坐标被认为是归一化坐标, 需要转换
    fn to_ac_rect(&self, size: Size) -> Rect {
        self.absolutized(size).unwrap()
    }
}

/// 转换为绝对坐标点
pub trait ToAcPoints {
    /// 转换为绝对坐标点
    fn to_ac_points(&self, size: Size) -> Points;
}

impl ToAcPoints for Points {
    /// 转换为绝对坐标, 整数坐标被认为是绝对坐标, 不需要转换
    fn to_ac_points(&self, _size: Size) -> Points {
        self.clone()
    }
}

impl ToAcPoints for PointFs {
    /// 转换为绝对坐标, 整数坐标被认为是归一化坐标, 需要转换
    fn to_ac_points(&self, size: Size) -> Points {
        self.iter().map(|p| p.to_ac_point(size)).collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_point() {
        let size = Size::new(10, 10);

        let pi = Point::new(1, 1);
        let pf = PointF::new(0.1, 0.1);

        assert_eq!(pi.to_nc_point(size), pf);
        assert_eq!(pf.to_nc_point(size), pf);

        assert_eq!(pi.to_ac_point(size), pi);
        assert_eq!(pf.to_ac_point(size), pi);
    }
    #[test]
    fn test_rect() {
        let size = Size::new(10, 10);

        let ri = Rect::new(1, 1, 1, 2);
        let rf = RectF::new(0.1, 0.1, 0.1, 0.2);

        assert_eq!(ri.to_nc_rect(size), rf);
        assert_eq!(rf.to_nc_rect(size), rf);

        assert_eq!(ri.to_ac_rect(size), ri);
        assert_eq!(rf.to_ac_rect(size), ri);
    }
}
