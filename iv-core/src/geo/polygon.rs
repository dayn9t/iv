use geo::Contains;
pub use geo::GeoNum;
use geo_types::{Coord, CoordNum, LineString, Polygon};
use rx_core::serde_export::{Deserialize, Serialize};

use crate::geo::{IShape, PointT, SizeT};

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

pub fn to_line_string<T: CoordNum>(points: &Vec<PointT<T>>) -> LineString<T> {
    let v = points.into_iter().map(|p| geo_coord(*p)).collect();
    LineString::<T>::new(v)
}

pub fn to_geo_point<T: CoordNum>(p: &PointT<T>) -> geo::Point<T> {
    geo::Point::<T>(p.to_tuple().into())
}

/// 多边形
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PolygonT<T: CoordNum>(Vec<PointT<T>>);

impl<T: GeoNum> PolygonT<T> {
    /// 获取顶点集合引用
    pub fn vertices_ref(&self) -> &Vec<PointT<T>> {
        &self.0
    }

    /// 转换成 ::geo 类型
    pub fn to_geo(&self) -> Polygon<T> {
        Polygon::<T>::new(to_line_string(&self.0), vec![])
    }

    /// 获取坐标归一化PolygonT
    pub fn normalized<T1: CoordNum, D: GeoNum>(&self, size: SizeT<T1>) -> Option<PolygonT<D>> {
        let ps: Vec<PointT<D>> = self.0.iter().map(|p| p.normalized(size).unwrap()).collect();
        Some(PolygonT::from(ps))
    }

    /// 获取坐标绝对化的PolygonT
    pub fn absolutized<T1: CoordNum, D: GeoNum>(&self, size: SizeT<T1>) -> Option<PolygonT<D>> {
        let ps: Vec<PointT<D>> = self
            .0
            .iter()
            .map(|p| p.absolutized(size).unwrap())
            .collect();
        Some(PolygonT::from(ps))
    }
}

impl<T: GeoNum> From<Vec<PointT<T>>> for PolygonT<T> {
    fn from(vertices: Vec<PointT<T>>) -> Self {
        Self(vertices)
    }
}

impl<T: GeoNum> Into<Vec<PointT<T>>> for PolygonT<T> {
    fn into(self) -> Vec<PointT<T>> {
        self.0
    }
}

impl<T: GeoNum> IPolygon<T> for PolygonT<T> {
    fn vertices(&self) -> Vec<PointT<T>> {
        self.0.clone()
    }
}

impl<T: GeoNum> IShape<T> for PolygonT<T> {
    fn area(&self) -> T {
        todo!()
    }

    fn perimeter(&self) -> T {
        todo!()
    }

    fn centroid(&self) -> PointT<T> {
        todo!()
    }

    fn contains(&self, point: &PointT<T>) -> bool {
        let polygon = self.to_geo();
        let p = to_geo_point(point);
        polygon.contains(&p)
    }
}

pub type PolygonF = PolygonT<f32>;

pub type PolygonI = PolygonT<i32>;

#[cfg(test)]
mod tests {
    use crate::geo::PointF;

    use super::*;

    #[test]
    fn it_works() {
        let ps = vec![
            PointF { x: 1.0, y: 1.0 },
            PointF { x: 3.0, y: 1.0 },
            PointF { x: 2.0, y: 2.0 },
            PointF { x: 3.0, y: 3.0 },
            PointF { x: 1.0, y: 3.0 },
        ];
        let poly = PolygonF::from(ps);
        // 外部点
        assert!(!poly.contains(&PointF::zero()));
        // 边界点
        assert!(!poly.contains(&PointF::one()));
        // 内部点
        assert!(poly.contains(&PointF { x: 1.1, y: 1.1 }));
        // 凹陷处, 外部点
        assert!(!poly.contains(&PointF { x: 3.0, y: 2.0 }));
    }
}
