use path_macro::path;
use rx_core::prelude::AnyResult;
use rx_core::sys::fs::to_string;
use std::path::Path;
use std::process::Command;
use tracing::*;

pub fn extract_images(video_file: &Path, dst_dir: &Path, fps: f32, ext: &str) {
    // 使用 ffmpeg 提取图片
    let mut cmd = Command::new("ffmpeg");
    let dst_file = to_string(path!(dst_dir / format!("%04d.{ext}")));
    let args = [
        "-i",
        video_file.to_str().unwrap(),
        "-vf",
        &format!("fps={fps}"),
        "-q:v",
        "2", // 质量参数: 1-最高，31-最低
        &dst_file,
    ];
    info!("ffmpeg {:?}", args.join(" "));
    cmd.args(&args);
    let output = cmd.output().unwrap();
    if !output.status.success() {
        error!("ffmpeg failed: {:?}", output);
    }
}

/// 从视频中提取流
pub fn extract_stream(input_file: &Path, dst_file: &Path, stream_id: usize) -> AnyResult<()> {
    let mut cmd = Command::new("ffmpeg");
    let args = [
        "-i",
        input_file.to_str().unwrap(),
        "-map",
        &format!("0:{stream_id}"),
        dst_file.to_str().unwrap(),
    ];
    info!("ffmpeg {:?}", args.join(" "));
    cmd.args(&args);
    let output = cmd.output().unwrap();
    if !output.status.success() {
        error!("ffmpeg failed: {:?}", output);
        return Err(anyhow::anyhow!("ffmpeg command failed"));
    }
    Ok(())
}
