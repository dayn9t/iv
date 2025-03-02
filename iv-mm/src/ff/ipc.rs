use anyhow::anyhow;
use rx_core::sys::fs::to_string;
use rx_core::text::AnyResult;
use serde_json::Value;
use std::path::Path;
use std::process::Command;

/// 获取视频文件的流数量
pub fn get_stream_count(video_file: &Path) -> AnyResult<usize> {
    let output = Command::new("ffprobe")
        .args(&[
            "-v",
            "error",
            "-show_entries",
            "stream",
            "-of",
            "json",
            &to_string(video_file),
        ])
        .output()?;

    if !output.status.success() {
        return Err(anyhow!(format!("ffprobe failed: {:?}", output)));
    }

    let stdout = String::from_utf8(output.stdout)?;
    let json: Value = serde_json::from_str(&stdout)?;
    let streams = json["streams"]
        .as_array()
        .ok_or(anyhow!("Failed to parse streams"))?;
    Ok(streams.len())
}
