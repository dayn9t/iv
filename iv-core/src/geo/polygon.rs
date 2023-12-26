use geo_types::CoordNum;

use crate::geo::{PointT, Shape};

/// 多边形
pub trait Polygon<T: CoordNum>: Shape<T> {
    /// 获取顶点集合
    fn vertices(&self) -> Vec<PointT<T>>;

    //fn intersects(&self, other: &Self) -> bool;
    //fn overlaps(&self, other: &Self) -> bool;
    //fn distance(&self, other: &Self) -> f64;
}