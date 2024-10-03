use iv_mm::meta::ExifManager;

fn main() {
    let file = "/home/jiang/rs/iv/data/image/jack2.jpg";
    let exif = ExifManager::new(file).unwrap();

    let size = exif.get_size();
    println!("Size: {size}");

    if let Some(gps) = exif.get_gps_coordinates() {
        println!("Location: {gps:?}");
    }

    if let Some(time) = exif.get_time() {
        println!("Time: {time}");
    }
}
