use crate::image::ocv::{
    CvPoint, CvScalar, cv_points, gray_as_mat, gray_as_mut_mat, image_as_mat, image_as_mut_mat,
};
use anyhow::anyhow;
use image::{DynamicImage, GrayImage, RgbImage};
use iv_core::geo::{PointFs, Points, Size, ToAcPoints};
use opencv::core::MatTraitConst;
use opencv::imgproc;
use rx_core::text::AnyResult;

/// 根据多边形区域，生成掩码图像
pub fn make_mask(size: Size, roi: &PointFs, color: u8) -> GrayImage {
    let roi = roi.to_ac_points(size);
    make_mask_i32(size, &roi, color)
}

/// 根据多边形区域，生成掩码图像
pub fn make_mask_i32(size: Size, roi: &Points, color: u8) -> GrayImage {
    let mut mask = GrayImage::new(size.width as u32, size.height as u32);

    let mut mat = gray_as_mut_mat(&mut mask);

    let roi_mat = cv_points(roi.clone());
    imgproc::fill_poly(
        &mut mat,
        &roi_mat,
        CvScalar::all(color as f64),
        8,
        0,
        CvPoint::new(0, 0),
    )
    .unwrap();
    mask
}

/// 复制掩码区域
pub fn copy_gray_masked(src: &GrayImage, dst: &mut GrayImage, mask: &GrayImage) -> AnyResult<()> {
    let src = gray_as_mat(src);
    let mut dst = gray_as_mut_mat(dst);
    let mask = gray_as_mat(mask);
    src.copy_to_masked(&mut dst, &mask)?;
    Ok(())
}

/// 复制掩码区域
pub fn copy_rgb_masked(src: &RgbImage, dst: &mut RgbImage, mask: &GrayImage) -> AnyResult<()> {
    let src = image_as_mat(src);
    let mut dst = image_as_mut_mat(dst);
    let mask = gray_as_mat(mask);
    src.copy_to_masked(&mut dst, &mask)?;
    Ok(())
}

/// 复制掩码区域
pub fn copy_image_masked(
    src: &DynamicImage,
    dst: &mut DynamicImage,
    mask: &GrayImage,
) -> AnyResult<()> {
    match src {
        DynamicImage::ImageLuma8(gray) => {
            if let DynamicImage::ImageLuma8(dst_gray) = dst {
                copy_gray_masked(gray, dst_gray, mask)?;
            } else {
                return Err(anyhow!(
                    "Destination image type does not match source image type"
                ));
            }
        }
        DynamicImage::ImageRgb8(rgb) => {
            if let DynamicImage::ImageRgb8(dst_rgb) = dst {
                copy_rgb_masked(rgb, dst_rgb, mask)?;
            } else {
                return Err(anyhow!(
                    "Destination image type does not match source image type"
                ));
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use path_macro::path;
    //use candle_core::Shape;
    use super::*;
    use crate::PACKAGE_DIR;
    use crate::image::{load_image, show, show_gray};
    use iv_core::geo::{Point, SIZE_NHD};

    #[test]
    fn test_mask() {
        let roi = vec![
            Point { x: 100, y: 100 },
            Point { x: 200, y: 100 },
            Point { x: 200, y: 200 },
        ];

        let mask = make_mask_i32(SIZE_NHD, &roi, 255);
        show_gray(&mask, "mask", 0);
    }

    #[test]
    fn test_mat_copy() {
        let file = path!(PACKAGE_DIR / "../assets/images/lena.jpg");

        println!("file: {}", file.display());
        let src = load_image(&file).unwrap();
        show(&src, "src", 0);

        let roi = vec![
            Point { x: 100, y: 100 },
            Point { x: 200, y: 100 },
            Point { x: 200, y: 200 },
        ];

        let size = Size {
            width: src.width() as i32,
            height: src.height() as i32,
        };
        let mask = make_mask_i32(size, &roi, 255);

        let dst = RgbImage::new(src.width(), src.height());
        let mut dst = DynamicImage::ImageRgb8(dst);
        copy_image_masked(&src, &mut dst, &mask).unwrap();

        show(&dst, "dst", 0);
    }
}
