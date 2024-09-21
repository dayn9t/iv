use std::error::Error;

fn main() {
    let file_name = "/home/jiang/rs/iv/data/1.jpg";
    match rexif::parse_file(&file_name) {
        Ok(exif) => {
            println!(
                "{} {} exif entries: {}",
                file_name,
                exif.mime,
                exif.entries.len()
            );

            for entry in &exif.entries {
                println!("    {}: {}", entry.tag, entry.value_more_readable);
            }
        }
        Err(e) => {
            eprintln!("Error in {}: {e}", &file_name);
        }
    }
}
