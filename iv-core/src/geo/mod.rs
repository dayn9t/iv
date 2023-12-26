pub use point::*;
pub use rect::*;
pub use size::*;
pub use polygon::*;
pub use shape::*;

use crate::valid_types;

mod point;
mod rect;
mod size;
mod polygon;
mod shape;

// 可用几何标量类型
valid_types!(ValidGeoType, i32, i64, f32, f64);

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
