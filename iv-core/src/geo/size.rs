use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::types::*;

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

    /// 检查是否包含另一个尺寸
    pub fn contains(self, other: Self) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    /// 将尺寸限制在当前边界内
    pub fn bound_size(self, other: Self) -> Self {
        if self.contains(other) { other } else { self }
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

impl<T> Display for SizeT<T>
where
    T: CoordNum + DivAssign + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

/// 2D尺寸(i32)
pub type Size = SizeT<i32>;
/// 2D尺寸(i32)
pub type SizeI = SizeT<i32>;
/// 2D尺寸(u32)
pub type SizeU = SizeT<u32>;
/// 2D尺寸(i64)
pub type SizeL = SizeT<i64>;
/// 2D尺寸(f32)
pub type SizeF = SizeT<f32>;
/// 2D尺寸(f64)
pub type SizeD = SizeT<f64>;

pub const SIZE_8K: Size = Size {
    width: 8192,
    height: 4320,
};
pub const SIZE_8K_UHD: Size = Size {
    width: 7680,
    height: 4320,
};
pub const SIZE_DCI_4K: Size = Size {
    width: 4096,
    height: 2160,
};
pub const SIZE_4K_UHD: Size = Size {
    width: 3840,
    height: 2160,
};
pub const SIZE_3K: Size = Size {
    width: 2880,
    height: 1620,
};
pub const SIZE_2K: Size = Size {
    width: 2048,
    height: 1080,
};
pub const SIZE_FHD: Size = Size {
    width: 1920,
    height: 1080,
};
pub const SIZE_QHD: Size = Size {
    width: 960,
    height: 540,
};
pub const SIZE_HD_PLUS: Size = Size {
    width: 1600,
    height: 900,
};
pub const SIZE_HD: Size = Size {
    width: 1280,
    height: 720,
};
pub const SIZE_NHD: Size = Size {
    width: 640,
    height: 360,
};
pub const SIZE_VGA: Size = Size {
    width: 640,
    height: 480,
};
pub const SIZE_QVGA: Size = Size {
    width: 320,
    height: 240,
};
pub const SIZE_PAL: Size = Size {
    width: 768,
    height: 576,
};
pub const SIZE_IM: Size = Size {
    width: 224,
    height: 224,
};

/// 深度学习检测器常规尺寸
pub const SIZE_DL224: Size = Size {
    width: 224,
    height: 224,
};

/// 深度学习检测器常规尺寸
pub const SIZE_DL640: Size = Size {
    width: 640,
    height: 640,
};
/// 深度学习检测器常规尺寸, 近似16:9, 32对齐
pub const SIZE_DL640X384: Size = Size {
    width: 640,
    height: 384,
};
/// 深度学习检测器常规尺寸, 近似16:9, 32对齐
pub const SIZE_DL640X352: Size = Size {
    width: 640,
    height: 352,
};

/// 深度学习检测器常规尺寸
pub const SIZE_DL720: Size = Size {
    width: 720,
    height: 720,
};

/// 深度学习检测器常规尺寸
pub const SIZE_DL1280: Size = Size {
    width: 1280,
    height: 1280,
};
