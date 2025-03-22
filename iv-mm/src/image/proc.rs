use std::path::Path;

use crate::image::ImageRgb;
use crate::image::ocv::{CvSize, image_as_mat, image_as_mut_mat, yuyv_as_mat2c};
use image::{DynamicImage, RgbImage, imageops};
use iv_core::geo::{Rect, RectF, Size};
use opencv::imgproc::{COLOR_YUV2RGB_YUYV, cvt_color};
use rx_core::sys::fs::make_parent;
use rx_core::text::AnyResult;

/// 加在图像
pub fn load_image(path: &Path) -> anyhow::Result<DynamicImage> {
    let im = image::ImageReader::open(&path)?.decode()?;
    Ok(im)
}

/// 保存图像
pub fn save_image(image: &DynamicImage, path: &Path) -> AnyResult<()> {
    make_parent(path)?;
    image.save(path)?;
    Ok(())
}

/// YUYV422 转化成 RGB
pub fn yuyv_to_rgb(buffer: &[u8], image: &mut RgbImage) {
    let size = Size {
        width: image.width() as i32,
        height: image.height() as i32,
    };
    let src = yuyv_as_mat2c(buffer, size);
    let mut dst = image_as_mut_mat(image);

    cvt_color(&src, &mut dst, COLOR_YUV2RGB_YUYV, 0).unwrap();
}

/// 获取图像区域
pub fn get_roi(image: &DynamicImage, rect: RectF) -> DynamicImage {
    let size = Size::new(image.width() as i32, image.height() as i32);
    let r = rect.absolutized(size).unwrap();
    let r = r & Rect::from_size(size);
    let r = r.to().unwrap();
    let sub = image.crop_imm(r.x, r.y, r.width, r.height);
    sub
}

/*
/// 设置图像区域
pub fn put_image(src: &RgbImage, dst: &RgbImage, x: u32, y: u32) -> DynamicImage {
    unimplemented!()
}*/

/// 获取图像区域
pub fn get_roi_i32(image: &DynamicImage, rect: Rect) -> DynamicImage {
    let size = Size::new(image.width() as i32, image.height() as i32);
    let r = rect & Rect::from_size(size);
    let r = r.to().unwrap();
    let sub = image.crop_imm(r.x, r.y, r.width, r.height);
    sub
}

/// 获取图像区域
pub fn get_roi_rgb_i32(image: &RgbImage, rect: Rect) -> RgbImage {
    let size = Size::new(image.width() as i32, image.height() as i32);
    let r = rect & Rect::from_size(size);
    let r = r.to().unwrap();
    let dst = imageops::crop_imm(image, r.x, r.y, r.width, r.height);
    dst.to_image()
}

/// 缩放推向到小于等于指定尺寸, 保持宽高比缩, 要求32对齐
pub fn resize_into_box(src: &DynamicImage, size: Size) -> DynamicImage {
    let aligned = 32;
    let (width, height) = {
        let w = src.width() as i32;
        let h = src.height() as i32;
        if w < h {
            let w = w * size.height / h + aligned - 1;
            (w / aligned * aligned, size.height)
        } else {
            let h = h * size.width / w + aligned - 1;
            (size.width, h / aligned * aligned)
        }
    };

    let dst = src.resize_exact(
        width as u32,
        height as u32,
        imageops::FilterType::CatmullRom,
    );
    dst
}

/// 缩放推向到指定尺寸, 不保持宽高比缩放
pub fn resize(src: &ImageRgb, dst: &mut ImageRgb) -> AnyResult<()> {
    let src_mat = image_as_mat(&src);
    let mut dst_mat = image_as_mut_mat(dst);
    let dst_size = CvSize::new(dst.width() as i32, dst.height() as i32);

    opencv::imgproc::resize(
        &src_mat,
        &mut dst_mat,
        dst_size,
        0.0,
        0.0,
        opencv::imgproc::INTER_CUBIC,
    )?;
    Ok(())
}

/// 缩放推向到指定尺寸, 保持宽高比缩放，TODO
pub fn resize_to1(src: &DynamicImage, size: Size) -> DynamicImage {
    let dst = src.resize_exact(
        size.width as u32,
        size.height as u32,
        imageops::FilterType::CatmullRom,
    );
    dst
}
/*
/// 图像(RGB)转换为张量(NCHW)
pub fn image_to_tensor(src: &DynamicImage, device: &Device) -> anyhow::Result<Tensor> {
    let assets = src.to_rgb8().into_raw();
    let tensor = Tensor::from_vec(assets, (src.height() as usize, src.width() as usize, 3), device)?
        .permute((2, 0, 1))?; // HWC -> CHW

    let tensor = (tensor.unsqueeze(0)?.to_dtype(DType::F32)? * (1. / 255.))?; // CHW -> NCHW
    Ok(tensor)
}
*/

#[cfg(test)]
mod tests {
    //use candle_core::Shape;
    use super::*;

    #[test]
    fn test_load_image() {
        let f = Path::new("/home/jiang/rs/ias/ias-dl/assets/cans.jpg");
        let im = load_image(&f).unwrap();
        assert_eq!(im.width(), 1920);
        assert_eq!(im.height(), 1080);
    }

    #[test]
    fn test_resize_into_box() {
        let f = Path::new("/home/jiang/rs/ias/ias-dl/assets/cans.jpg");
        let src = load_image(&f).unwrap();

        assert_eq!(src.width(), 1920);
        assert_eq!(src.height(), 1080);

        let dst = resize_into_box(&src, Size::new(640, 640));
        assert_eq!(dst.width(), 640);
        assert_eq!(dst.height(), 384); // not 360, FIXME: 352更接近, 且垂直压缩
    }
    /*
    #[test]
    fn test_image_to_tensor() {
        let f = Path::new("/home/jiang/rs/ias/ias-dl/assets/cans.jpg");
        let src = load_image(&f).unwrap();

        let dst = image_to_tensor(&src, &Device::Cpu).unwrap();
        let shape = Shape::from(vec![1, 3, 1080, 1920]);
        assert_eq!(dst.shape(), &shape);
    }*/
}
