use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use num::{NumCast, ToPrimitive};
use serde::{Deserialize, Serialize};

use super::{PointT, ValidGeoType};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SizeT<T: ValidGeoType> {
    pub width: T,
    pub height: T,
}

impl<T: ValidGeoType> SizeT<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    pub fn from_point(pt: PointT<T>) -> Self
        where
            T: ValidGeoType,
    {
        Self {
            width: pt.x,
            height: pt.y,
        }
    }

    pub fn area(self) -> T {
        self.width * self.height
    }

    pub fn empty(self) -> bool {
        self.width <= T::zero() || self.height <= T::zero()
    }

    pub fn to<D: ValidGeoType + NumCast>(self) -> Option<SizeT<D>>
        where
            T: ToPrimitive,
    {
        Some(SizeT {
            width: D::from(self.width)?,
            height: D::from(self.height)?,
        })
    }
}

impl<T> Add for SizeT<T>
    where
        T: ValidGeoType + AddAssign,
{
    type Output = SizeT<T>;

    fn add(mut self, rhs: SizeT<T>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T> Sub for SizeT<T>
    where
        T: ValidGeoType + SubAssign,
{
    type Output = SizeT<T>;

    fn sub(mut self, rhs: SizeT<T>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T> Mul<T> for SizeT<T>
    where
        T: ValidGeoType + MulAssign,
{
    type Output = SizeT<T>;

    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T> Div<T> for SizeT<T>
    where
        T: ValidGeoType + DivAssign,
{
    type Output = SizeT<T>;

    fn div(mut self, rhs: T) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<T> AddAssign for SizeT<T>
    where
        T: ValidGeoType + AddAssign,
{
    fn add_assign(&mut self, rhs: SizeT<T>) {
        self.width += rhs.width;
        self.height += rhs.height;
    }
}

impl<T> SubAssign for SizeT<T>
    where
        T: ValidGeoType + SubAssign,
{
    fn sub_assign(&mut self, rhs: SizeT<T>) {
        self.width -= rhs.width;
        self.height -= rhs.height;
    }
}

impl<T> MulAssign<T> for SizeT<T>
    where
        T: ValidGeoType + MulAssign,
{
    fn mul_assign(&mut self, rhs: T) {
        self.width *= rhs;
        self.height *= rhs;
    }
}

impl<T> DivAssign<T> for SizeT<T>
    where
        T: ValidGeoType + DivAssign,
{
    fn div_assign(&mut self, rhs: T) {
        self.width /= rhs;
        self.height /= rhs;
    }
}
