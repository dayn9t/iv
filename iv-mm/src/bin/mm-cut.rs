use clap::Parser;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

/// Command line arguments structure
#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// Path to the media file
    file_path: String,

    /// Segment length in seconds
    #[clap(short, long)]
    segment_length: f64,

    /// Overlap time in seconds
    #[clap(short, long, default_value = "0.0")]
    overlap: f64,
}

fn get_media_duration(file_path: &str) -> Result<f64, String> {
    let output = Command::new("ffprobe")
        .args(&[
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            file_path,
        ])
        .output()
        .map_err(|e| format!("Failed to execute ffprobe: {}", e))?;

    let duration_str = String::from_utf8_lossy(&output.stdout);
    let duration = f64::from_str(duration_str.trim())
        .map_err(|e| format!("Failed to parse duration: {}", e))?;

    Ok(duration)
}

fn split_media_file(file_path: &str, segment_length: f64, overlap: f64) -> Result<(), String> {
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
    let mut segment_length = segment_length;

    while start_time < duration {
        let output_file = format!("{}_{}.{}", file_stem, segment_index, file_extension);
        println!("#{} {}", segment_index, output_file);
        Command::new("ffmpeg")
            .args(&[
                "-i",
                file_path,
                "-ss",
                &start_time.to_string(),
                "-t",
                &segment_length.to_string(),
                "-c",
                "copy",
                "-y",
                &output_file,
            ])
            .output()
            .map_err(|e| format!("Failed to execute ffmpeg: {}", e))?;

        start_time += segment_length - overlap;
        if segment_index == 0 {
            segment_length += overlap;
        }
        segment_index += 1;
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    match split_media_file(&args.file_path, args.segment_length, args.overlap) {
        Ok(_) => println!("File split successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }
}
