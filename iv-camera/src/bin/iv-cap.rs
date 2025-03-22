use image::RgbImage;

use iv_camera::{ActiveCamera, CameraManager, Model, PIXEL_FORMAT_YUYV};
use iv_core::geo::{Rect, SIZE_NHD};
use iv_mm::image::{get_roi_rgb_i32, yuyv_to_rgb};

fn main() {
    let manager = CameraManager::new().unwrap();
    let cameras = manager.cameras();
    // 程序选择第一个摄像头
    let camera = cameras.get(0).expect("No cameras found");
    let cam_model = camera.properties().get::<Model>().unwrap();
    println!("Using camera: {}", *cam_model);

    let mut camera = ActiveCamera::new(&camera, PIXEL_FORMAT_YUYV, SIZE_NHD, 30.0);
    let mut buffer = Vec::new();

    let mut image = RgbImage::new(640, 360);
    let rect = Rect {
        x: (640 - 360) / 2,
        y: 0,
        width: 360,
        height: 360,
    };

    camera.start();
    for i in 0..1000 {
        camera.read(&mut buffer);
        yuyv_to_rgb(&buffer, &mut image);

        let roi = get_roi_rgb_i32(&image, rect);
        //show_rgb(&roi, "BGR Image", 20);
        let p = format!("{}.jpg", i);
        println!("save: {}", &p);
        roi.save(&p).unwrap()
    }
}
