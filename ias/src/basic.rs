/// Map定义
pub use std::collections::BTreeMap as Map;
pub use std::io::Result as IoResult;
pub use std::path::{Path, PathBuf};

pub use rx_core::fs::*;
pub use rx_core::text::*;
pub use rx_core::time::*;
pub use uuid::Uuid;

pub use iv_core::geo::*;

/// 感兴趣区域 - 绝对坐标（i32）
pub type Roi = Points;

/// 感兴趣区域 - 相对坐标（f64）
pub type RoiD = PointDs;

/// 浮点取值范围
pub type RangeD = std::ops::Range<f64>;

/// 获取对象Uuid
pub trait ToUuid {
    fn to_uuid(&self) -> Uuid;
}
