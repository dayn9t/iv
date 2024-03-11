use geo::{Contains, GeoNum};
use geo_types::{Coord, CoordNum, LineString, Polygon};

use crate::geo::{IShape, PointT};

/// 多边形
pub trait IPolygon<T: CoordNum>: IShape<T> {
    /// 获取顶点集合
    fn vertices(&self) -> Vec<PointT<T>>;

    //fn intersects(&self, other: &Self) -> bool;
    //fn overlaps(&self, other: &Self) -> bool;
    //fn distance(&self, other: &Self) -> f64;
}

pub fn geo_coord<T: CoordNum>(p: PointT<T>) -> Coord<T> {
    Coord { x: p.x, y: p.y }
}

pub fn to_line_string<T: CoordNum>(points: Vec<PointT<T>>) -> LineString<T> {
    let v = points.into_iter().map(|p| geo_coord(p)).collect();
    LineString::<T>::new(v)
}

pub fn geo_point<T: CoordNum>(p: PointT<T>) -> geo::Point<T> {
    geo::Point::<T>(p.to_tuple().into())
}

/// 判断点是否在多边形内, 边界点不算内部
pub fn contains<T: GeoNum>(points: &Vec<PointT<T>>, point: PointT<T>) -> bool {
    let polygon = Polygon::<T>::new(to_line_string(points.clone()), vec![]);
    let p = geo_point(point);
    polygon.contains(&p)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geo::PointF;

    #[test]
    fn it_works() {
        let ps = vec![
            PointF { x: 1.0, y: 1.0 },
            PointF { x: 3.0, y: 1.0 },
            PointF { x: 2.0, y: 2.0 },
            PointF { x: 3.0, y: 3.0 },
            PointF { x: 1.0, y: 3.0 },
        ];
        // 外部点
        assert!(!contains(&ps, PointF::zero()));
        // 边界点
        assert!(!contains(&ps, PointF::one()));
        // 内部点
        assert!(contains(&ps, PointF { x: 1.1, y: 1.1 }));
        // 凹陷处, 外部点
        assert!(!contains(&ps, PointF { x: 3.0, y: 2.0 }));
    }
}
