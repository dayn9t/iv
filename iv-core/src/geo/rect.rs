use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

use num::{NumCast, ToPrimitive};
use serde::{Deserialize, Serialize};

use super::{PointT, SizeT, ValidGeoType};

#[inline(always)]
fn partial_min<T: PartialOrd>(a: T, b: T) -> T {
    if a <= b {
        a
    } else {
        b
    }
}

#[inline(always)]
fn partial_max<T: PartialOrd>(a: T, b: T) -> T {
    if b >= a {
        b
    } else {
        a
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct RectT<T: ValidGeoType> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T: ValidGeoType> RectT<T> {
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn one() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            width: T::one(),
            height: T::one(),
        }
    }

    pub fn from_point_size(pt: PointT<T>, sz: SizeT<T>) -> Self
        where
            T: ValidGeoType + ValidGeoType,
    {
        Self::new(pt.x, pt.y, sz.width, sz.height)
    }

    pub fn from_points(pt1: PointT<T>, pt2: PointT<T>) -> Self
        where
            T: ValidGeoType,
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

    /// 获取左上坐标
    pub fn left_top(&self) -> PointT<T>
        where
            T: ValidGeoType,
    {
        PointT::new(self.x, self.y)
    }

    /// 获取右上坐标
    pub fn right_top(&self) -> PointT<T>
        where
            T: ValidGeoType,
    {
        PointT::new(self.x + self.width, self.y)
    }

    /// 获取右下坐标
    pub fn right_bottom(&self) -> PointT<T>
        where
            T: ValidGeoType,
    {
        PointT::new(self.x + self.width, self.y + self.height)
    }

    /// 获取左下坐标
    pub fn left_bottom(&self) -> PointT<T>
        where
            T: ValidGeoType,
    {
        PointT::new(self.x, self.y + self.height)
    }


    /// 获取尺寸
    pub fn size(&self) -> SizeT<T>
        where
            T: ValidGeoType,
    {
        SizeT::new(self.width, self.height)
    }

    /// 获取面积
    pub fn area(&self) -> T {
        self.width * self.height
    }

    /// 判定区域是否为空
    pub fn empty(&self) -> bool {
        self.width <= T::zero() || self.height <= T::zero()
    }

    /// 是否包含
    pub fn contains(&self, pt: PointT<T>) -> bool
        where
            T: ValidGeoType,
    {
        self.x <= pt.x
            && pt.x < self.x + self.width
            && self.y <= pt.y
            && pt.y < self.y + self.height
    }

    /// 获取转换类型
    pub fn to<D: ValidGeoType + NumCast>(&self) -> Option<RectT<D>>
        where
            T: ToPrimitive,
    {
        Some(RectT {
            x: D::from(self.x)?,
            y: D::from(self.y)?,
            width: D::from(self.width)?,
            height: D::from(self.height)?,
        })
    }

    /// 获取定点坐标
    pub fn vertexes(&self) -> Vec<PointT<T>> {
        vec![self.left_top(), self.right_top(), self.right_bottom(), self.left_bottom()]
    }

    /// 获取归一化后的RectT
    pub fn normalized(&self, size: SizeT<T>) -> Self {
        Self {
            x: self.x / size.width,
            y: self.y / size.height,
            width: self.width / size.width,
            height: self.height / size.height,
        }
    }
}

impl<P, R> Add<PointT<P>> for RectT<R>
    where
        P: ValidGeoType,
        R: ValidGeoType + AddAssign<P>,
{
    type Output = RectT<R>;

    fn add(mut self, rhs: PointT<P>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<P, R> Sub<PointT<P>> for RectT<R>
    where
        P: ValidGeoType,
        R: ValidGeoType + SubAssign<P>,
{
    type Output = RectT<R>;

    fn sub(mut self, rhs: PointT<P>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<S, R> Add<SizeT<S>> for RectT<R>
    where
        S: ValidGeoType,
        R: ValidGeoType + AddAssign<S>,
{
    type Output = RectT<R>;

    fn add(mut self, rhs: SizeT<S>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<S, R> Sub<SizeT<S>> for RectT<R>
    where
        S: ValidGeoType,
        R: ValidGeoType + SubAssign<S>,
{
    type Output = RectT<R>;

    fn sub(mut self, rhs: SizeT<S>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T: ValidGeoType> BitOr for RectT<T> {
    type Output = RectT<T>;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}

impl<T: ValidGeoType> BitAnd for RectT<T> {
    type Output = RectT<T>;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}

impl<P, R> AddAssign<PointT<P>> for RectT<R>
    where
        P: ValidGeoType,
        R: ValidGeoType + AddAssign<P>,
{
    fn add_assign(&mut self, rhs: PointT<P>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<P, R> SubAssign<PointT<P>> for RectT<R>
    where
        P: ValidGeoType,
        R: ValidGeoType + SubAssign<P>,
{
    fn sub_assign(&mut self, rhs: PointT<P>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<S, R> AddAssign<SizeT<S>> for RectT<R>
    where
        S: ValidGeoType,
        R: ValidGeoType + AddAssign<S>,
{
    fn add_assign(&mut self, rhs: SizeT<S>) {
        self.width += rhs.width;
        self.height += rhs.height;
    }
}

impl<S, R> SubAssign<SizeT<S>> for RectT<R>
    where
        S: ValidGeoType,
        R: ValidGeoType + SubAssign<S>,
{
    fn sub_assign(&mut self, rhs: SizeT<S>) {
        self.width -= rhs.width;
        self.height -= rhs.height;
    }
}

impl<T: ValidGeoType> BitOrAssign for RectT<T> {
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

impl<T: ValidGeoType> BitAndAssign for RectT<T> {
    fn bitand_assign(&mut self, rhs: Self) {
        let x1 = partial_max(self.x, rhs.x);
        let y1 = partial_max(self.y, rhs.y);
        self.width = partial_min(self.x + self.width, rhs.x + rhs.width) - x1;
        self.height = partial_min(self.y + self.height, rhs.y + rhs.height) - y1;
        self.x = x1;
        self.y = y1;
        if self.empty() {
            *self = RectT::default();
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
