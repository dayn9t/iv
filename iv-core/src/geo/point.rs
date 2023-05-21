use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use num::{NumCast, ToPrimitive};
use serde::{Deserialize, Serialize};

use super::{RectT, SizeT, ValidGeoType};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PointT<T: ValidGeoType> {
    pub x: T,
    pub y: T,
}

impl<T: ValidGeoType> PointT<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn from_size(sz: SizeT<T>) -> Self
    where
        T: ValidGeoType,
    {
        Self {
            x: sz.width,
            y: sz.height,
        }
    }

    pub fn cross(self, pt: PointT<T>) -> f64
    where
        f64: From<T>,
    {
        let self_x: f64 = From::from(self.x);
        let self_y: f64 = From::from(self.y);
        let pt_x: f64 = From::from(pt.x);
        let pt_y: f64 = From::from(pt.y);
        self_x * pt_y - self_y * pt_x
    }

    pub fn dot(self, pt: PointT<T>) -> T {
        self.x * pt.x + self.y * pt.y
    }

    pub fn ddot(self, pt: PointT<T>) -> f64
    where
        f64: From<T>,
    {
        let self_x: f64 = From::from(self.x);
        let self_y: f64 = From::from(self.y);
        let pt_x: f64 = From::from(pt.x);
        let pt_y: f64 = From::from(pt.y);
        self_x * pt_x + self_y * pt_y
    }

    pub fn inside(self, rect: RectT<T>) -> bool
    where
        T: ValidGeoType,
    {
        rect.contains(self)
    }

    pub fn norm(self) -> f64
    where
        f64: From<T>,
    {
        let self_x: f64 = From::from(self.x);
        let self_y: f64 = From::from(self.y);
        (self_x.powi(2) + self_y.powi(2)).sqrt()
    }

    pub fn to<D: ValidGeoType + NumCast>(self) -> Option<PointT<D>>
    where
        T: ToPrimitive,
    {
        Some(PointT {
            x: D::from(self.x)?,
            y: D::from(self.y)?,
        })
    }
}

impl<T: ValidGeoType> From<SizeT<T>> for PointT<T> {
    fn from(s: SizeT<T>) -> Self {
        Self {
            x: s.width,
            y: s.height,
        }
    }
}

impl<T> Add for PointT<T>
where
    T: ValidGeoType + AddAssign,
{
    type Output = PointT<T>;

    fn add(mut self, rhs: PointT<T>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T> Sub for PointT<T>
where
    T: ValidGeoType + SubAssign,
{
    type Output = PointT<T>;

    fn sub(mut self, rhs: PointT<T>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T> Mul<T> for PointT<T>
where
    T: ValidGeoType + MulAssign,
{
    type Output = PointT<T>;

    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T> Mul<PointT<T>> for PointT<T>
where
    T: ValidGeoType + MulAssign,
{
    type Output = PointT<T>;

    fn mul(self, rhs: PointT<T>) -> Self::Output {
        PointT {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Div<T> for PointT<T>
where
    T: ValidGeoType + DivAssign,
{
    type Output = PointT<T>;

    fn div(mut self, rhs: T) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<T> AddAssign for PointT<T>
where
    T: ValidGeoType + AddAssign,
{
    fn add_assign(&mut self, rhs: PointT<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> SubAssign for PointT<T>
where
    T: ValidGeoType + SubAssign,
{
    fn sub_assign(&mut self, rhs: PointT<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> MulAssign<T> for PointT<T>
where
    T: ValidGeoType + MulAssign,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T> DivAssign<T> for PointT<T>
where
    T: ValidGeoType + DivAssign,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
