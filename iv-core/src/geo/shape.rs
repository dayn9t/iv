use crate::geo::PointT;
use geo_types::CoordNum;

/// 形状
pub trait IShape<T: CoordNum> {
    /// 获取形状多边形面积
    fn area(&self) -> T;
    /// 获取形状周长
    fn perimeter(&self) -> T;
    /// 获取形状重心
    fn centroid(&self) -> PointT<T>;
    /// 判断点是否在形状内
    fn contains(&self, point: &PointT<T>) -> bool;
    //fn intersects(&self, other: &Self) -> bool;
    //fn overlaps(&self, other: &Self) -> bool;
    //fn distance(&self, other: &Self) -> f64;
}
