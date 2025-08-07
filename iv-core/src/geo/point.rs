use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::types::*;
use rx_core::m::{div_f64, mul_round_f64};

use super::{IShape, RectT, SizeT};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PointT<T: CoordNum> {
    pub x: T,
    pub y: T,
}

impl<T: CoordNum> PointT<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }

    pub fn one() -> Self {
        Self {
            x: T::one(),
            y: T::one(),
        }
    }

    pub fn from_size(sz: SizeT<T>) -> Self
    where
        T: CoordNum,
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

    pub fn inside(self, rect: &RectT<T>) -> bool
    where
        T: CoordNum,
    {
        rect.contains(&self)
    }

    pub fn norm(self) -> f64
    where
        f64: From<T>,
    {
        let self_x: f64 = From::from(self.x);
        let self_y: f64 = From::from(self.y);
        (self_x.powi(2) + self_y.powi(2)).sqrt()
    }

    pub fn to<D: CoordNum>(self) -> Option<PointT<D>> {
        Some(PointT {
            x: D::from(self.x)?,
            y: D::from(self.y)?,
        })
    }

    /// 获取 Tuple
    pub fn to_tuple(&self) -> (T, T) {
        (self.x, self.y)
    }

    /// 获取坐标归一化PointT
    pub fn normalized<T1: CoordNum, D: CoordNum>(&self, size: SizeT<T1>) -> Option<PointT<D>> {
        let x = div_f64(self.x, size.width)?;
        let y = div_f64(self.y, size.height)?;
        Some(PointT { x, y })
    }

    /// 获取坐标绝对化的PointT
    pub fn absolutized<T1: CoordNum, D: CoordNum>(&self, size: SizeT<T1>) -> Option<PointT<D>> {
        let x = mul_round_f64(self.x, size.width)?;
        let y = mul_round_f64(self.y, size.height)?;
        Some(PointT { x, y })
    }
}

impl<T: CoordNum> From<SizeT<T>> for PointT<T> {
    fn from(s: SizeT<T>) -> Self {
        Self {
            x: s.width,
            y: s.height,
        }
    }
}

impl<T> Add for PointT<T>
where
    T: CoordNum + AddAssign,
{
    type Output = PointT<T>;

    fn add(mut self, rhs: PointT<T>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T> Sub for PointT<T>
where
    T: CoordNum + SubAssign,
{
    type Output = PointT<T>;

    fn sub(mut self, rhs: PointT<T>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T> Mul<T> for PointT<T>
where
    T: CoordNum + MulAssign,
{
    type Output = PointT<T>;

    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T> Mul<PointT<T>> for PointT<T>
where
    T: CoordNum + MulAssign,
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
    T: CoordNum + DivAssign,
{
    type Output = PointT<T>;

    fn div(mut self, rhs: T) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<T> AddAssign for PointT<T>
where
    T: CoordNum + AddAssign,
{
    fn add_assign(&mut self, rhs: PointT<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> SubAssign for PointT<T>
where
    T: CoordNum + SubAssign,
{
    fn sub_assign(&mut self, rhs: PointT<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> MulAssign<T> for PointT<T>
where
    T: CoordNum + MulAssign,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T> DivAssign<T> for PointT<T>
where
    T: CoordNum + DivAssign,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
