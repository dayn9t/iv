use std::process::Command;
use std::str::FromStr;
use std::path::Path;

fn get_media_duration(file_path: &str) -> Result<f64, String> {
    let output = Command::new("ffprobe")
        .args(&[
            "-v", "error",
            "-show_entries", "format=duration",
            "-of", "default=noprint_wrappers=1:nokey=1",
            file_path,
        ])
        .output()
        .map_err(|e| format!("Failed to execute ffprobe: {}", e))?;

    let duration_str = String::from_utf8_lossy(&output.stdout);
    let duration = f64::from_str(duration_str.trim())
        .map_err(|e| format!("Failed to parse duration: {}", e))?;

    Ok(duration)
}

fn split_media_file(file_path: &str, segment_length: f64) -> Result<(), String> {
    let duration = get_media_duration(file_path)?;
    let file_stem = Path::new(file_path)
        .file_stem()
        .ok_or("Invalid file path")?
        .to_str()
        .ok_or("Failed to convert file stem to string")?;
    let file_extension = Path::new(file_path)
        .extension()
        .ok_or("Invalid file path")?
        .to_str()
        .ok_or("Failed to convert file extension to string")?;

    let mut start_time = 0.0;
    let mut segment_index = 0;

    while start_time < duration {
        let output_file = format!("{}_part{}.{}", file_stem, segment_index, file_extension);
        Command::new("ffmpeg")
            .args(&[
                "-i", file_path,
                "-ss", &start_time.to_string(),
                "-t", &segment_length.to_string(),
                "-c", "copy",
                &output_file,
            ])
            .output()
            .map_err(|e| format!("Failed to execute ffmpeg: {}", e))?;

        start_time += segment_length;
        segment_index += 1;
    }

    Ok(())
}

fn main() {
    let file_path = "/home/jiang/py/asr/asr1/audio/德云社_01-03-06.mp3";
    let segment_length = 600.0; // 10 minutes

    match split_media_file(file_path, segment_length) {
        Ok(_) => println!("File split successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }
}