pub use point::*;
pub use rect::*;
pub use size::*;

use crate::valid_types;

mod point;
mod rect;
mod size;

/// 可用几何标量类型
valid_types!(ValidGeoType, i32, i64, f32, f64);
