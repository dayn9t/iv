use iv_mm::IV_MM_DIR;
use iv_mm::meta::ExifManager;
use path_macro::path;
use rexiv2::GpsInfo;
use rx_core::prelude::{Deserialize, Serialize};
use rx_core::text::json;
use rx_core::time::now;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SnapshotMeta {
    /// 组编号
    pub group: u32,
    /// 抓图源编号
    pub source: u32,
    /// 摄像机编号
    pub camera: u32,
    /// 任务
    pub task: u32,
}

fn main() {
    //let file = "/home/jiang/rs/iv/assets/images/jack2.jpg";
    let dir = path!(IV_MM_DIR / "../assets/images");

    let src_file = path!(dir / "jack2.jpg");
    let dst_file = path!(dir / "jack2.jpg");
    //let dst_file = path!("/tmp/iv/jack2.jpg");

    let mut exif = ExifManager::new(src_file).unwrap();

    let size = exif.get_size();
    println!("Size: {size}");

    if let Some(gps) = exif.get_gps_coordinates() {
        println!("Location: {gps:?}");
    }

    if let Some(time) = exif.get_time() {
        println!("Time: {time}");
    }

    let gps = GpsInfo {
        latitude: 37.0,
        longitude: -122.0,
        altitude: 0.0,
    };
    exif.set_gps_coordinates(gps).unwrap();

    let time = now();
    exif.set_time(time).unwrap();

    let meta = SnapshotMeta {
        group: 1,
        source: 3,
        camera: 4,
        task: 5,
    };
    let meta = json::to_string(&meta).unwrap();

    exif.set_comment(&meta).unwrap();

    exif.save(dst_file).unwrap();
}
