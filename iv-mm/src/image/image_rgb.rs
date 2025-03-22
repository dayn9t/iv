use crate::image::{IImage2D, Rgb};
use derive_more::{AsRef, Deref, DerefMut, From, Into};

use image::{RgbImage, imageops};
use iv_core::geo::iface::ISize2D;
use iv_core::geo::{Rect, Size};
use rx_core::sys::fs::make_parent;
use rx_core::text::AnyResult;
use std::path::Path;

pub fn cvt_color(rgb: Rgb) -> image::Rgb<u8> {
    image::Rgb(rgb.into())
}

/// 动态图像包装类
///
/// TODO: 1.关注 imageops::replace 等函数，2.关注
///
#[derive(Deref, DerefMut, From, Into, AsRef, Clone)]
pub struct ImageRgb(image::RgbImage);

impl ISize2D<i32> for ImageRgb {
    fn width(&self) -> i32 {
        self.0.width() as i32
    }

    fn height(&self) -> i32 {
        self.0.height() as i32
    }
}

impl IImage2D for ImageRgb {
    fn get_roi(&self, rect: Rect) -> Self {
        let self_rect = Rect::from_size(self.size());

        let r = rect & self_rect;
        let r = r.to().unwrap();
        assert!(r.width > 0 && r.height > 0);

        let dst = imageops::crop_imm(&self.0, r.x, r.y, r.width, r.height);
        Self(dst.to_image())
    }

    fn fill_color(&mut self, color: Rgb) {
        let color = cvt_color(color);
        for pixel in self.0.pixels_mut() {
            *pixel = color;
        }
    }

    fn resize_to(&self, dst: &mut Self) {
        let w = dst.width() as u32;
        let h = dst.height() as u32;
        let resized = imageops::resize(&self.0, w, h, imageops::FilterType::CatmullRom);
        dst.0 = resized;
    }

    fn load(path: &Path) -> AnyResult<Self> {
        let im = image::ImageReader::open(&path)?.decode()?;
        Ok(ImageRgb(im.into_rgb8()))
    }

    fn save(&self, path: &Path) -> AnyResult<()> {
        make_parent(path)?;
        self.0.save(path)?;
        Ok(())
    }
}

impl ImageRgb {
    pub fn new(size: Size, color: Rgb) -> Self {
        let im = image::ImageBuffer::from_fn(size.width as u32, size.height as u32, |_, _| {
            cvt_color(color)
        });

        ImageRgb(im)
    }

    pub fn new_black(size: Size) -> Self {
        Self(RgbImage::new(size.width as u32, size.height as u32))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_mat() {}
}
