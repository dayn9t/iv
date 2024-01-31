use std::fmt::{Debug};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use geo_types::CoordNum;
use serde::{Deserialize, Serialize};

use super::PointT;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SizeT<T: CoordNum> {
    pub width: T,
    pub height: T,
}

impl<T: CoordNum> SizeT<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    pub fn from_point(pt: PointT<T>) -> Self
        where
            T: CoordNum,
    {
        Self {
            width: pt.x,
            height: pt.y,
        }
    }

    /// 获取简短信息
    pub fn brief(&self) -> String {
        format!("{:?}x{:?}", self.width, self.height)
    }

    /// 获取面积
    pub fn area(self) -> T {
        self.width * self.height
    }

    /// FIXME: 有问题
    pub fn empty(self) -> bool {
        self.width <= T::zero() || self.height <= T::zero()
    }

    pub fn to<D: CoordNum>(self) -> Option<SizeT<D>> {
        Some(SizeT {
            width: D::from(self.width)?,
            height: D::from(self.height)?,
        })
    }
}


impl<T> Add for SizeT<T>
    where
        T: CoordNum + AddAssign,
{
    type Output = SizeT<T>;

    fn add(mut self, rhs: SizeT<T>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T> Sub for SizeT<T>
    where
        T: CoordNum + SubAssign,
{
    type Output = SizeT<T>;

    fn sub(mut self, rhs: SizeT<T>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T> Mul<T> for SizeT<T>
    where
        T: CoordNum + MulAssign,
{
    type Output = SizeT<T>;

    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T> Div<T> for SizeT<T>
    where
        T: CoordNum + DivAssign,
{
    type Output = SizeT<T>;

    fn div(mut self, rhs: T) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<T> AddAssign for SizeT<T>
    where
        T: CoordNum + AddAssign,
{
    fn add_assign(&mut self, rhs: SizeT<T>) {
        self.width += rhs.width;
        self.height += rhs.height;
    }
}

impl<T> SubAssign for SizeT<T>
    where
        T: CoordNum + SubAssign,
{
    fn sub_assign(&mut self, rhs: SizeT<T>) {
        self.width -= rhs.width;
        self.height -= rhs.height;
    }
}

impl<T> MulAssign<T> for SizeT<T>
    where
        T: CoordNum + MulAssign,
{
    fn mul_assign(&mut self, rhs: T) {
        self.width *= rhs;
        self.height *= rhs;
    }
}

impl<T> DivAssign<T> for SizeT<T>
    where
        T: CoordNum + DivAssign,
{
    fn div_assign(&mut self, rhs: T) {
        self.width /= rhs;
        self.height /= rhs;
    }
}
