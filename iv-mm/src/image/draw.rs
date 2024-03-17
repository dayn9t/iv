use image::{DynamicImage, GenericImageView, RgbImage};
use opencv::imgproc::rectangle;
use opencv::prelude::*;

use iv_core::geo::{PolygonF, PolygonI, RectF, SizeT};

use crate::image::color::Rgb;

/// 绘制矩形
pub fn draw_rect(image: &mut RgbImage, rect: RectF, color: Rgb, thickness: i32) {
    let size = SizeT {
        width: image.width(),
        height: image.height(),
    };
/*
    rectangle(image, rect, color, thickness).expect("TODO: panic message");

    let mut image = DynamicImage::ImageRgb8(image.clone());
    let mut r = rect.absolutized(size).unwrap();
    r.dilate_me(-(thickness - 1) / 2);
    for _i in 0..thickness {
        //ip::draw_hollow_rect_mut(&mut image, ip_rect(&r), ip_rgb(color));
        r = r.dilate(1);
    }*/
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
        //ip::draw_hollow_ellipse_mut(image, c, r.width / 2, r.height / 2, ip_rgb(color));
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
    //let polygon = ip_polygon(&polygon);

    for _i in 0..thickness {
        //ip::draw_polygon_mut(image, &polygon, ip_rgb(color));
        // TODO: thickness
    }
}
