use std::path::Path;
use std::time::Instant;

use image::{DynamicImage, RgbImage};
use iv_audio_io::play_sound_wait;
use iv_camera::{ActiveCamera, CameraManager, Model, PIXEL_FORMAT_YUYV};
use iv_core::geo::SIZE_DL224;
use iv_core::geo::{Rect, SIZE_NHD};
use iv_mm::image::{get_roi_rgb_i32, show_rgb, yuyv_to_rgb};
use path_macro::path;
use rx_core::log::{info, init_log};
use rx_core::sys::fs::make_parent;
use rx_core::time::local_time_id;

use ias_dl::cls::{C2dModelCfg, Classifier2D};
use ias_dl::common::Device;
use ias_dl_onnx::c2d::ClassifierY8;

/// 有条件播放
fn if_play(silent: bool, dir: &Path, file: &str) {
    if !silent {
        play_sound_wait(&path!(dir / file));
    }
}

pub fn main() -> anyhow::Result<()> {
    init_log(1);

    let fps = 0.25;
    let silent = false;
    let conf = 0.6;
    let sounds = path!("/home/jiang/ws/bot/sounds");

    if_play(silent, &sounds, "app-start.wav");

    let cfg = C2dModelCfg {
        name: "sort".to_string(),
        input_size: SIZE_DL224,
        input_batch: 1,
        num_classes: 6,
        conf_thr: 0.3,
        normalized: true,
        profile: false,
    };

    let model_path = path!("/home/jiang/ws/bot/sort/model_dir");
    let snapshot_dir = path!("/home/jiang/ws/bot/snapshot");

    let device = Device::Cpu;
    info!("loaded model {:?}", &model_path);
    let model = ClassifierY8::load(cfg, &model_path, &device)?;

    let manager = CameraManager::new().unwrap();
    let cameras = manager.cameras();
    // 程序选择第一个摄像头
    let camera = match cameras.get(0) {
        None => {
            if_play(silent, &sounds, "camera-loss.wav");
            panic!("无法找到哦摄像机")
        }
        Some(c) => c,
    };
    let cam_model = camera.properties().get::<Model>().unwrap();
    println!("Using camera: {}", *cam_model);

    let mut buffer = Vec::new();
    let mut image = RgbImage::new(640, 360);
    let rect = Rect {
        x: (640 - 360) / 2,
        y: 0,
        width: 360,
        height: 360,
    };

    let mut camera = ActiveCamera::new(&camera, PIXEL_FORMAT_YUYV, SIZE_NHD, fps);
    camera.start();
    for _j in 0..100000 {
        if_play(silent, &sounds, "classifier-start.wav");

        for _i in 0..5 {
            camera.read(&mut buffer);
            yuyv_to_rgb(&buffer, &mut image);

            let roi = get_roi_rgb_i32(&image, rect);
            //            show_rgb(&roi, "BGR Image", 20);

            let roi = DynamicImage::ImageRgb8(roi);

            let start = Instant::now();
            let res = model.forward(&roi)?;
            let duration = Instant::now() - start;
            info!("    duration: {:?}", duration);
            info!("    {:?}", res.top());

            let idx = res.top().class_index;
            if res.top().confidence > conf && idx < 5 {
                let file = format!("class{}.wav", idx);
                if_play(silent, &sounds, &file);
            }

            let file = path!(snapshot_dir / idx.to_string() / local_time_id() + ".jpg");
            info!("    {:?}", file);
            make_parent(&file).unwrap();
            roi.save(file).unwrap()
        }
    }
    Ok(())
}
