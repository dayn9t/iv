use rexiv2::GpsInfo;
use rx_core::time::now;
use iv_mm::meta::ExifManager;

fn main() {
    //let file = "/home/jiang/rs/iv/data/image/jack2.jpg";
    let file = "/home/jiang/1/s4/0001.jpg";
    let dst = "/home/jiang/1/s4/0001a.jpg";
    let mut exif = ExifManager::new(file).unwrap();

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

    exif.save(file).unwrap();
}
