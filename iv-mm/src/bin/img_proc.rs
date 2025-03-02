use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, Rgb, RgbImage};
use iv_mm::image::show;

fn create_white_image() -> DynamicImage {
    let width = 640;
    let height = 1280;
    let white_pixel = Rgb([255, 0, 0]);
    let mut img = RgbImage::new(width, height);

    for pixel in img.pixels_mut() {
        *pixel = white_pixel;
    }

    DynamicImage::ImageRgb8(img)
}

fn main() {
    let image = create_white_image();
    // Save or use the white_image as needed
    show(&image, "White Image", 0);
    let image = image.resize(640, 360, FilterType::CatmullRom);

    show(&image, "White Image", 0);
    println!("{:?}", image.dimensions());
}
