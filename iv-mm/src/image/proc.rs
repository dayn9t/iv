use std::path::Path;

use image::DynamicImage;
use iv_core::geo::{Rect, RectF, Size};

/// 加在图像
pub fn load_image(path: &Path) -> anyhow::Result<DynamicImage> {
    let im = image::io::Reader::open(&path)?.decode()?;
    Ok(im)
}

pub fn get_roi(image: &DynamicImage, rect: RectF) -> DynamicImage {
    let size = Size::new(image.width() as i32, image.height() as i32);
    let r = rect.absolutized(size).unwrap();
    let r = r & Rect::from_size(size);
    let r = r.to().unwrap();
    let sub = image.crop_imm(r.x, r.y, r.width, r.height);
    sub
}

/// 缩放推向到小于等于指定尺寸, 保持宽高比缩
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
        image::imageops::FilterType::CatmullRom,
    );
    dst
}

/// 缩放推向到指定尺寸, 不保持宽高比缩放
pub fn resize_to(src: &DynamicImage, size: Size) -> DynamicImage {
    let dst = src.resize_exact(
        size.width as u32,
        size.height as u32,
        image::imageops::FilterType::CatmullRom,
    );
    dst
}
/*
/// 图像(RGB)转换为张量(NCHW)
pub fn image_to_tensor(src: &DynamicImage, device: &Device) -> anyhow::Result<Tensor> {
    let data = src.to_rgb8().into_raw();
    let tensor = Tensor::from_vec(data, (src.height() as usize, src.width() as usize, 3), device)?
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

        let dst = resize_into_box(&src, Size::new(640, 640));
        assert_eq!(dst.width(), 640);
        assert_eq!(dst.height(), 352); // not 360
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
