pub use point::*;
pub use polygon::*;
pub use rect::*;
pub use shape::*;
pub use size::*;

mod point;
mod rect;
mod size;
mod polygon;
mod shape;

/// 2D点(i32)
pub type Point = PointT<i32>;
/// 2D点(i32)
pub type PointI = PointT<i32>;
/// 2D点(i64)
pub type PointL = PointT<i64>;
/// 2D点(f32)
pub type PointF = PointT<f32>;
/// 2D点(f64)
pub type PointD = PointT<f64>;

/// 2D点集(i32)
pub type Points = Vec<Point>;
/// 2D点集(i64)
pub type PointLs = Vec<PointL>;
/// 2D点集(f32)
pub type PointFs = Vec<PointF>;
/// 2D点集(f64)
pub type PointDs = Vec<PointD>;

/// 2D尺寸(i32)
pub type Size = SizeT<i32>;
/// 2D尺寸(i32)
pub type SizeI = SizeT<i32>;
/// 2D尺寸(i64)
pub type SizeL = SizeT<i64>;
/// 2D尺寸(f32)
pub type SizeF = SizeT<f32>;
/// 2D尺寸(f64)
pub type SizeD = SizeT<f64>;

/// 长方形(i32)
pub type Rect = RectT<i32>;
/// 长方形(i32)
pub type RectI = RectT<i32>;
/// 长方形(i64)
pub type RectL = RectT<i64>;
/// 长方形(f32)
pub type RectF = RectT<f32>;
/// 长方形(f64)
pub type RectD = RectT<f64>;


pub const SIZE_8K: Size = Size { width: 8192, height: 4320 };
pub const SIZE_8K_UHD: Size = Size { width: 7680, height: 4320 };
pub const SIZE_DCI_4K: Size = Size { width: 4096, height: 2160 };
pub const SIZE_4K_UHD: Size = Size { width: 3840, height: 2160 };
pub const SIZE_3K: Size = Size { width: 2880, height: 1620 };
pub const SIZE_2K: Size = Size { width: 2048, height: 1080 };
pub const SIZE_FHD: Size = Size { width: 1920, height: 1080 };
pub const SIZE_QHD: Size = Size { width: 960, height: 540 };
pub const SIZE_HD_PLUS: Size = Size { width: 1600, height: 900 };
pub const SIZE_HD: Size = Size { width: 1280, height: 720 };
pub const SIZE_NHD: Size = Size { width: 640, height: 360 };
pub const SIZE_VGA: Size = Size { width: 640, height: 480 };
pub const SIZE_QVGA: Size = Size { width: 320, height: 240 };
pub const SIZE_PAL: Size = Size { width: 768, height: 576 };
pub const SIZE_IM: Size = Size { width: 224, height: 224 };

/// 深度学习检测器常规尺寸
pub const SIZE_DL224: Size = Size { width: 224, height: 224 };

/// 深度学习检测器常规尺寸
pub const SIZE_DL640: Size = Size { width: 640, height: 640 };

/// 深度学习检测器常规尺寸
pub const SIZE_DL720: Size = Size { width: 720, height: 720 };

/// 深度学习检测器常规尺寸
pub const SIZE_DL1280: Size = Size { width: 1280, height: 1280 };
