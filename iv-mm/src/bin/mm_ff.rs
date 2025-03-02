use iv_mm::ff::get_stream_count;
use path_macro::path;

fn main() {
    let video_file = path!("/mnt/temp/2025_02_13/C1_2025_02_10T13_32_44.mkv");
    //let video_file = path!("/mnt/temp/2025_02_13/C1_2025_02_11T10_38_40_L.mkv");
    match get_stream_count(&video_file) {
        Ok(count) => println!("Total number of streams: {}", count),
        Err(e) => eprintln!("Error: {}", e),
    }
}
