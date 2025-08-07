use super::types::*;
use derive_more::{Deref, DerefMut};
use geo::Contains;
pub use geo::GeoNum;
use geo_types::{Coord, LineString, Polygon as GtPolygon};
use rx_core::m::{partial_max, partial_min};

use crate::geo::{IShape, PointT, RectT, SizeT};

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
#[derive(Debug, Default, Clone, Serialize, Deserialize, Deref, DerefMut)]
pub struct PolygonT<T: CoordNum>(Vec<PointT<T>>);

impl<T: GeoNum> PolygonT<T> {
    /// 获取顶点集合引用
    pub fn vertices_ref(&self) -> &Vec<PointT<T>> {
        &self.0
    }

    /// 转换成 ::geo 类型
    pub fn to_geo(&self) -> GtPolygon<T> {
        GtPolygon::<T>::new(to_line_string(&self.0), vec![])
    }

    /// 判断多边形是否为矩形
    pub fn is_rect(&self) -> bool {
        self.0.len() == 4
            && self.0[0].x == self.0[1].x
            && self.0[1].y == self.0[2].y
            && self.0[2].x == self.0[3].x
            && self.0[3].y == self.0[0].y
    }

    /// 如果多边形是矩形, 则获取矩形
    pub fn to_rect(&self) -> Option<RectT<T>> {
        if self.is_rect() {
            Some(RectT::from_points(self.0[0], self.0[2]))
        } else {
            None
        }
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

impl<T: GeoNum> From<RectT<T>> for PolygonT<T> {
    fn from(rect: RectT<T>) -> Self {
        Self(rect.vertices())
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

/// 获取点集合的边界框
pub fn bounding_box<T: GeoNum>(points: &[PointT<T>]) -> RectT<T> {
    let mut min_x = points[0].x;
    let mut min_y = points[0].y;
    let mut max_x = points[0].x;
    let mut max_y = points[0].y;

    for p in &points[1..] {
        min_x = partial_min(min_x, p.x);
        min_y = partial_min(min_y, p.y);
        max_x = partial_max(max_x, p.x);
        max_y = partial_max(max_y, p.y);
    }

    RectT::new(min_x, min_y, max_x - min_x, max_y - min_y)
}

/// 多边形(i32)
pub type Polygon = PolygonT<i32>;
/// 多边形(i32)
pub type PolygonI = PolygonT<i32>;
/// 多边形(i64)
pub type PolygonL = PolygonT<i64>;
/// 多边形(f32)
pub type PolygonF = PolygonT<f32>;
/// 多边形(f64)
pub type PolygonD = PolygonT<f64>;

#[cfg(test)]
mod tests {
    use crate::geo::{PointF, Rect};
    use rx_core::text::json;

    use super::*;

    #[test]
    fn test_contains() {
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

    #[test]
    fn test_bounding_box() {
        let points = vec![
            PointF { x: 1.0, y: 1.0 },
            PointF { x: 3.0, y: 1.0 },
            PointF { x: 2.0, y: 2.0 },
            PointF { x: 3.0, y: 3.0 },
            PointF { x: 0.0, y: 4.0 },
        ];
        let bbox = bounding_box(&points);
        assert_eq!(bbox.x, 0.0);
        assert_eq!(bbox.y, 1.0);
        assert_eq!(bbox.right(), 3.0);
        assert_eq!(bbox.bottom(), 4.0);
    }

    #[test]
    fn test_io() {
        let p: PolygonI = Rect::one().into();
        let s = json::to_pretty(&p).unwrap();
        println!("polygon: {}", s);
    }
}
