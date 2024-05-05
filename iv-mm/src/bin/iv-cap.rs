use libcamera::camera_manager::CameraManager;
use libcamera::properties;
use opencv::highgui;

use iv_core::geo::SIZE_NHD;
use iv_mm::camera::{ActiveCamera, PIXEL_FORMAT_YUYV};
use iv_mm::image::ocv::yuyv_to_mat3c;

fn main() {
    let manager = CameraManager::new().unwrap();
    let cameras = manager.cameras();
    // 程序选择第一个摄像头
    let camera = cameras.get(0).expect("No cameras found");
    let cam_model = camera.properties().get::<properties::Model>().unwrap();
    println!("Using camera: {}", *cam_model);

    let mut camera = ActiveCamera::new(&camera, PIXEL_FORMAT_YUYV, SIZE_NHD, 5.0);
    let mut buffer = Vec::new();

    camera.start();
    for _i in 0..1000 {
        camera.read(&mut buffer);

        let mat = yuyv_to_mat3c(&buffer, SIZE_NHD);
        highgui::imshow("BGR Image", &mat).unwrap();
        highgui::wait_key(30).unwrap();
    }
}
