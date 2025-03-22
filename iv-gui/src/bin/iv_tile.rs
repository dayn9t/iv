use image::{DynamicImage, GenericImage, GenericImageView, RgbImage, RgbaImage};
use opencv::core::{Mat, MatTrait, MatTraitManual, Size, ToInputArray};
use opencv::highgui;
use opencv::prelude::*;

fn tile_images(
    images: Vec<DynamicImage>,
    rows: usize,
    cols: usize,
) -> Result<Mat, Box<dyn std::error::Error>> {
    if images.is_empty() {
        return Err("No images provided".into());
    }

    let (img_width, img_height) = images[0].dimensions();
    let canvas_width = img_width * cols as u32;
    let canvas_height = img_height * rows as u32;

    let mut canvas = RgbImage::new(canvas_width, canvas_height);

    for (i, img) in images.iter().enumerate() {
        let rgb_img = img.to_rgb8();
        let x = (i % cols) as u32 * img_width;
        let y = (i / cols) as u32 * img_height;
        canvas.copy_from(&rgb_img, x, y)?;
    }

    let mat = Mat::from_slice(canvas.as_flat_samples().as_slice())?;
    Ok(mat.try_clone().unwrap())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths = vec![
        "/home/jiang/rs/iv/assets/images/lena.jpg",
        "/home/jiang/rs/iv/assets/images/lena.jpg",
        "/home/jiang/rs/iv/assets/images/lena.jpg",
        "/home/jiang/rs/iv/assets/images/lena.jpg",
    ];

    let images: Vec<DynamicImage> = paths.iter().map(|p| image::open(p).unwrap()).collect();

    let mat = tile_images(images, 2, 2)?;

    let window = "Tiled Images";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    highgui::imshow(window, &mat)?;
    highgui::wait_key(0)?;

    Ok(())
}
