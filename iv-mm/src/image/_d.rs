use image::{imageops, GenericImageView, RgbImage, DynamicImage};
use iv_core::geo::{Point, PolygonF, PolygonI, Rect, RectF, RectT, SizeT};

use crate::image::color::Rgb;

mod ip {
    // [imageproc文档](https://docs.rs/imageproc/0.23.0/imageproc/)
    pub use imageproc::drawing::draw_hollow_ellipse_mut;
    pub use imageproc::drawing::draw_hollow_rect_mut;
    pub use imageproc::drawing::draw_line_segment_mut;
    pub use imageproc::point::Point;
    pub use imageproc::rect::Rect;

    pub type Rgb = image::Rgb<u8>;
}

/// Point转换
fn ip_point(p: &Point) -> ip::Point<i32> {
    ip::Point { x: p.x, y: p.y }
}

/// Rect准换
fn ip_rect(r: &Rect) -> ip::Rect {
    ip::Rect::at(r.x, r.y).of_size(r.width as u32, r.height as u32)
}

fn ip_polygon(polygeon: &PolygonI) -> Vec<ip::Point<i32>> {
    polygeon
        .vertices_ref()
        .iter()
        .map(|p| ip_point(p))
        .collect()
}

/// 颜色准换
fn ip_rgb(rgb: Rgb) -> ip::Rgb {
    image::Rgb(rgb.rgb())
}

/*
/// 颜色距离
fn dist3(a: [u8; 3], b: Rgb) -> i64 {
    let b = b.rgb();
    let mut s = 0;
    for i in 0..3 {
        let d = a[i] as i64 - b[i] as i64;
        s += d * d;
    }
    s
}
/// 色度距离
fn dist_hua(a: [u8; 3], b: Rgb) -> i32 {
    let d = get_hua(b.rgb()) - get_hua(a);
    d.abs()
}*/

/*
/// 获取色度
fn get_hua(c: [u8; 3]) -> i32 {
    rgb_to_hsi(c)[0] as i32
}

/// H 的值在 0 到 360 之间，表示色相；S 的值在 0 到 1 之间，表示饱和度；V 的值在 0 到 255 之间，表示亮度。
fn rgb_to_hsi(c: [u8; 3]) -> [f32; 3] {
    let rgb = colorsys::Rgb::from((c[0], c[1], c[2]));
    let hsi = colorsys::Hsl::from(rgb);
    [
        hsi.hue() as f32,
        hsi.saturation() as f32,
        hsi.lightness() as f32,
    ]
}


/// 获取区域内最匹配的颜色
pub fn rect_color(image: &mut RgbImage, rect: RectF, colors: &[Rgb]) -> usize {
    //let huas: Vec<i64> = colors.iter().map(|c| get_hua(c.rgb())).collect();

    let size = SizeT {
        width: image.width(),
        height: image.height(),
    };
    let r = rect.absolutized(size).unwrap();
    let r = r & RectT::from_size(size);
    let sub = imageops::crop(image, r.x, r.y, r.width, r.height);
    let mut hists = vec![0; 12];
    for x in 0..r.width {
        for y in 0..r.height {
            let p = sub.get_pixel(x, y).0;

            let h = get_hua(p) / 30;
            if h >= 0 && h < 12 {
            } else {
                let _a = rgb_to_hsi(p);
                println!("{}", h);
            }
            assert!(h >= 0 && h < 12);
            hists[h as usize] += 1;
        }
    }
    println!("hist: {:?}", &hists);
    let zone = max_value_index(&hists) as i32;
    println!("color zone: {}", zone);
    let diffs: Vec<i32> = colors
        .iter()
        .map(|c| (get_hua(c.rgb()) / 30 - zone).abs())
        .collect();
    let color_index = min_value_index(&diffs);
    color_index
}

*/
/// 最大值的索引
pub fn max_value_index<T: Ord>(vec: &Vec<T>) -> usize {
    vec.iter()
        .enumerate()
        .max_by_key(|&(_, x)| x)
        .map(|(i, _)| i)
        .unwrap()
}

/// 最小值的索引
pub fn min_value_index<T: Ord>(vec: &Vec<T>) -> usize {
    vec.iter()
        .enumerate()
        .min_by_key(|&(_, x)| x)
        .map(|(i, _)| i)
        .unwrap()
}


/// 获取区域内最匹配的颜色
pub fn rect_color1(image: &mut RgbImage, rect: RectF, _colors: &[Rgb]) -> Rgb {
    let size = SizeT {
        width: image.width(),
        height: image.height(),
    };
    let r = rect.absolutized(size).unwrap();
    let r = r & RectT::from_size(size);
    let sub = imageops::crop(image, r.x, r.y, r.width, r.height);

    let x = r.width / 2;
    let y = r.height / 2;
    let p = sub.get_pixel(x, y).0;
    Rgb::new(p[0], p[1], p[2])
}

/// 绘制矩形
pub fn rectangle(image: &mut RgbImage, rect: RectF, color: Rgb, thickness: i32) {
    let size = SizeT {
        width: image.width(),
        height: image.height(),
    };
    let mut image = DynamicImage::ImageRgb8(image.clone());
    let mut r = rect.absolutized(size).unwrap();
    r.dilate_me(-(thickness - 1) / 2);
    for _i in 0..thickness {
        ip::draw_hollow_rect_mut(&mut image, ip_rect(&r), ip_rgb(color));
        r = r.dilate(1);
    }
}

/// 绘制椭圆
pub fn ellipse(image: &mut RgbImage, rect: RectF, color: Rgb, thickness: i32) {
    let size = SizeT {
        width: image.width(),
        height: image.height(),
    };
    let mut r = rect.absolutized(size).unwrap();
    r.dilate_me(-(thickness - 1) / 2);
    for _i in 0..thickness {
        let c = r.center().to_tuple();
        ip::draw_hollow_ellipse_mut(image, c, r.width / 2, r.height / 2, ip_rgb(color));
        r = r.dilate(1);
    }
}

/// 绘制多边形
pub fn polygon(image: &mut RgbImage, polygon: &PolygonF, color: Rgb, thickness: i32) {
    let size = SizeT {
        width: image.width(),
        height: image.height(),
    };
    let polygon: PolygonI = polygon.absolutized(size).unwrap();
    let polygon = ip_polygon(&polygon);

    for _i in 0..thickness {
        //ip::draw_polygon_mut(image, &polygon, ip_rgb(color));
        // TODO: thickness
    }
}
