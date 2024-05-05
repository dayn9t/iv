use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

use geo_types::CoordNum;
use rx_core::m::{div_f64, mul_round_f64, partial_max, partial_min};
use serde::{Deserialize, Serialize};

use super::{IPolygon, IShape, PointT, SizeT};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct RectT<T: CoordNum> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T: CoordNum> RectT<T> {
    /// 构建 - 从左, 上, 右, 下
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// 构建 - 在原点
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            width: T::zero(),
            height: T::zero(),
        }
    }

    /// 构建 - 左上在原点, 边长为1的矩形
    pub fn one() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            width: T::one(),
            height: T::one(),
        }
    }

    /// 构建 - 从尺寸
    pub fn from_size(sz: SizeT<T>) -> Self
    where
        T: CoordNum + CoordNum,
    {
        Self::new(T::zero(), T::zero(), sz.width, sz.height)
    }

    /// 构建 - 从左上, 尺寸
    pub fn from_point_size(pt: PointT<T>, sz: SizeT<T>) -> Self
    where
        T: CoordNum + CoordNum,
    {
        Self::new(pt.x, pt.y, sz.width, sz.height)
    }

    /// 构建 - 从(左,上),(右,下)
    pub fn from_points(pt1: PointT<T>, pt2: PointT<T>) -> Self
    where
        T: CoordNum,
    {
        let x = partial_min(pt1.x, pt2.x);
        let y = partial_min(pt1.y, pt2.y);
        Self::new(
            x,
            y,
            partial_max(pt1.x, pt2.x) - x,
            partial_max(pt1.y, pt2.y) - y,
        )
    }

    /// 构建 - 从左,上,右,下
    pub fn from_ltrb(x0: T, y0: T, x1: T, y1: T) -> Self
    where
        T: CoordNum,
    {
        Self::new(x0, y0, x1 - x0, y1 - y0)
    }

    /// 获取中心
    pub fn center(&self) -> PointT<T> {
        let two = T::one() + T::one();
        let x = self.x + self.width / two;
        let y = self.y + self.height / two;
        PointT { x, y }
    }

    /// 获取左上坐标
    pub fn left_top(&self) -> PointT<T> {
        PointT::new(self.x, self.y)
    }

    /// 获取右上坐标
    pub fn right_top(&self) -> PointT<T>
    where
        T: CoordNum,
    {
        PointT::new(self.x + self.width, self.y)
    }

    /// 获取右下坐标
    pub fn right_bottom(&self) -> PointT<T>
    where
        T: CoordNum,
    {
        PointT::new(self.x + self.width, self.y + self.height)
    }

    /// 获取左下坐标
    pub fn left_bottom(&self) -> PointT<T>
    where
        T: CoordNum,
    {
        PointT::new(self.x, self.y + self.height)
    }

    /// 获取尺寸
    pub fn size(&self) -> SizeT<T>
    where
        T: CoordNum,
    {
        SizeT::new(self.width, self.height)
    }

    /// 判定区域是否为空
    pub fn empty(&self) -> bool {
        self.width <= T::zero() || self.height <= T::zero()
    }

    /// 获取转换类型
    pub fn to<D: CoordNum>(&self) -> Option<RectT<D>> {
        Some(RectT {
            x: D::from(self.x)?,
            y: D::from(self.y)?,
            width: D::from(self.width)?,
            height: D::from(self.height)?,
        })
    }

    /// 向四边膨胀指定值
    pub fn dilate_me(&mut self, n: T) {
        self.x = self.x - n;
        self.y = self.y - n;
        self.width = self.width + n + n;
        self.height = self.height + n + n;
    }

    /// 向四边膨胀指定值
    pub fn dilate(&self, n: T) -> Self {
        let mut r = *self;
        r.dilate_me(n);
        r
    }
    /*
    def erode_me(self, n: Real) -> None:
        ///向四边腐蚀指定值///
        self.dilate_me(-n)

    def erode(self, n: Real) -> Self:
        ///向四边腐蚀指定值///
        return self.dilate(-n)

    def round_me(self) -> None:
        ///近似成整数///
        self.x = round(self.x)
        self.y = round(self.y)
        self.width = round(self.width)
        self.height = round(self.height)

    */

    /// 获取坐标归一化后的RectT
    pub fn normalized<T1: CoordNum, D: CoordNum>(&self, size: SizeT<T1>) -> Option<RectT<D>> {
        let x = div_f64(self.x, size.width)?;
        let y = div_f64(self.y, size.height)?;
        let width = div_f64(self.width, size.width)?;
        let height = div_f64(self.height, size.height)?;
        Some(RectT {
            x,
            y,
            width,
            height,
        })
    }

    /// 获取坐标绝对化的RectT
    pub fn absolutized<T1: CoordNum, D: CoordNum>(&self, size: SizeT<T1>) -> Option<RectT<D>> {
        let x = mul_round_f64(self.x, size.width)?;
        let y = mul_round_f64(self.y, size.height)?;
        let width = mul_round_f64(self.width, size.width)?;
        let height = mul_round_f64(self.height, size.height)?;
        Some(RectT {
            x,
            y,
            width,
            height,
        })
    }

    /// 计算交并比
    pub fn iou(self, other: Self) -> f64 {
        let s0 = (self & other).area().to_f64().unwrap();
        let s1 = (self | other).area().to_f64().unwrap();
        if s0 == 0.0 {
            0.0
        } else {
            s0 / s1
        }
    }

    /// 宽高比
    pub fn aspect_ratio(&self) -> f64 {
        self.width.to_f64().unwrap() / self.height.to_f64().unwrap()
    }
}

impl<T: CoordNum> IShape<T> for RectT<T> {
    /// 获取面积
    fn area(&self) -> T {
        self.width * self.height
    }

    /// 获取周长
    fn perimeter(&self) -> T {
        let two = T::one() + T::one();
        two * (self.width + self.height)
    }

    /// 获取重心
    fn centroid(&self) -> PointT<T> {
        self.center()
    }

    /// 判定是否包含指定点
    fn contains(&self, pt: &PointT<T>) -> bool {
        self.x <= pt.x
            && pt.x < self.x + self.width
            && self.y <= pt.y
            && pt.y < self.y + self.height
    }
}

impl<T: CoordNum> IPolygon<T> for RectT<T> {
    /// 获取顶点坐标数组
    fn vertices(&self) -> Vec<PointT<T>> {
        vec![
            self.left_top(),
            self.right_top(),
            self.right_bottom(),
            self.left_bottom(),
        ]
    }
}

/// 平移
impl<P, R> Add<PointT<P>> for RectT<R>
where
    P: CoordNum,
    R: CoordNum + AddAssign<P>,
{
    type Output = RectT<R>;

    fn add(mut self, rhs: PointT<P>) -> Self::Output {
        self += rhs;
        self
    }
}

/// 平移 - 反向
impl<P, R> Sub<PointT<P>> for RectT<R>
where
    P: CoordNum,
    R: CoordNum + SubAssign<P>,
{
    type Output = RectT<R>;

    fn sub(mut self, rhs: PointT<P>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<S, R> Add<SizeT<S>> for RectT<R>
where
    S: CoordNum,
    R: CoordNum + AddAssign<S>,
{
    type Output = RectT<R>;

    fn add(mut self, rhs: SizeT<S>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<S, R> Sub<SizeT<S>> for RectT<R>
where
    S: CoordNum,
    R: CoordNum + SubAssign<S>,
{
    type Output = RectT<R>;

    fn sub(mut self, rhs: SizeT<S>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T: CoordNum> BitOr for RectT<T> {
    type Output = RectT<T>;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}

impl<T: CoordNum> BitAnd for RectT<T> {
    type Output = RectT<T>;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}

/// 平移
impl<P, R> AddAssign<PointT<P>> for RectT<R>
where
    P: CoordNum,
    R: CoordNum + AddAssign<P>,
{
    fn add_assign(&mut self, rhs: PointT<P>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

/// 平移 - 反向
impl<P, R> SubAssign<PointT<P>> for RectT<R>
where
    P: CoordNum,
    R: CoordNum + SubAssign<P>,
{
    fn sub_assign(&mut self, rhs: PointT<P>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

/// 增加尺寸
impl<S, R> AddAssign<SizeT<S>> for RectT<R>
where
    S: CoordNum,
    R: CoordNum + AddAssign<S>,
{
    fn add_assign(&mut self, rhs: SizeT<S>) {
        self.width += rhs.width;
        self.height += rhs.height;
    }
}

/// 增加尺寸 - 反向
impl<S, R> SubAssign<SizeT<S>> for RectT<R>
where
    S: CoordNum,
    R: CoordNum + SubAssign<S>,
{
    fn sub_assign(&mut self, rhs: SizeT<S>) {
        self.width -= rhs.width;
        self.height -= rhs.height;
    }
}

/// 并集
impl<T: CoordNum> BitOrAssign for RectT<T> {
    fn bitor_assign(&mut self, rhs: Self) {
        if self.empty() {
            *self = rhs;
        } else if !rhs.empty() {
            let x1 = partial_min(self.x, rhs.x);
            let y1 = partial_min(self.y, rhs.y);
            self.width = partial_max(self.x + self.width, rhs.x + rhs.width) - x1;
            self.height = partial_max(self.y + self.height, rhs.y + rhs.height) - y1;
            self.x = x1;
            self.y = y1;
        }
    }
}

/// 交集
impl<T: CoordNum> BitAndAssign for RectT<T> {
    fn bitand_assign(&mut self, rhs: Self) {
        let x1 = partial_max(self.x, rhs.x);
        let y1 = partial_max(self.y, rhs.y);
        self.width = partial_min(self.x + self.width, rhs.x + rhs.width) - x1;
        self.height = partial_min(self.y + self.height, rhs.y + rhs.height) - y1;
        self.x = x1;
        self.y = y1;
        if self.empty() {
            *self = RectT::zero();
        }
    }
}
/*
impl fmt::Debug for RotatedRect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RotatedRect")
            .field("angle", &self.angle().map_err(|_| fmt::Error)?)
            .field("center", &self.center().map_err(|_| fmt::Error)?)
            .field("size", &self.size().map_err(|_| fmt::Error)?)
            .finish()
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs() {
        let size = SizeT {
            width: 640,
            height: 360,
        };
        let d = 1.0 / 3.0;
        let rect = RectT {
            x: d,
            y: 0.0,
            width: d,
            height: 1.0,
        };
        let rect: RectT<i32> = rect.absolutized(size).unwrap();
        print!("{:?}", &rect);
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_partial() {
        assert_eq!(1., partial_min(1., 2.));
        assert_eq!(1., partial_min(2., 1.));
        assert_eq!(1., partial_min(1., 1.));
        assert_eq!(1, partial_min(1, 2));
        assert_eq!(1, partial_min(2, 1));
        assert_eq!(1, partial_min(1, 1));

        assert_eq!(2., partial_max(1., 2.));
        assert_eq!(2., partial_max(2., 1.));
        assert_eq!(2., partial_max(2., 2.));
        assert_eq!(2, partial_max(1, 2));
        assert_eq!(2, partial_max(2, 1));
        assert_eq!(2, partial_max(2, 2));
    }

    #[test]
    fn test_bitor() {
        let mut r10 = RectT::new(0, 0, 10, 10);
        assert_eq!(r10 | r10, r10);

        let r2 = RectT::new(5, 5, 10, 10);
        r10 |= r2;
        assert_eq!(r10, RectT::new(0, 0, 15, 15));

        let r1 = RectT::one();
        let _r = r1 & r2;
        let _r = r1 & r2;
    }
}
