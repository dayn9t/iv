pub use point::*;
pub use rect::*;
pub use size::*;

use crate::valid_types;

mod point;
mod rect;
mod size;

// 可用几何标量类型
valid_types!(ValidGeoType, i32, i64, f32, f64);

/// 2D点(i32)
pub type Point = PointT<i32>;
/// 2D点(i64)
pub type PointL = PointT<i64>;
/// 2D点(f32)
pub type PointF = PointT<f32>;
/// 2D点(f64)
pub type PointD = PointT<f64>;

/// 2D尺寸(i32)
pub type Size = SizeT<i32>;
/// 2D尺寸(i64)
pub type SizeL = SizeT<i64>;
/// 2D尺寸(f32)
pub type SizeF = SizeT<f32>;
/// 2D尺寸(f64)
pub type SizeD = SizeT<f64>;

/// 长方形(i32)
pub type Rect = RectT<i32>;
/// 长方形(i64)
pub type RectL = RectT<i64>;
/// 长方形(f32)
pub type RectF = RectT<f32>;
/// 长方形(f64)
pub type RectD = RectT<f64>;
