use anyhow::anyhow;
use rx_core::sys::fs::to_string;
use rx_core::text::{AnyResult, json};
use serde_json::Value;
use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FfStreamData {
    pub streams: Vec<FfStreamInfo>,
}

impl FfStreamData {
    pub fn video_streams(&self) -> Vec<&FfStreamInfo> {
        self.streams
            .iter()
            .filter(|s| s.codec_type == "video")
            .collect()
    }

    pub fn audio_streams(&self) -> Vec<&FfStreamInfo> {
        self.streams
            .iter()
            .filter(|s| s.codec_type == "audio")
            .collect()
    }

    /// 根据title查找流
    pub fn find_stream_by_title(&self, title: &str) -> Option<&FfStreamInfo> {
        self.streams.iter().find(|s| {
            if let Some(t) = &s.tags.title {
                t == title
            } else {
                false
            }
        })
    }
    /// 根据title查找流索引
    pub fn stream_index_of_title(&self, title: &str) -> Option<usize> {
        let s = self.find_stream_by_title(title);
        match s {
            Some(stream) => Some(stream.index as usize),
            None => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FfStreamInfo {
    pub index: u32,
    pub codec_name: String,
    pub codec_type: String,
    pub tags: FfStreamTags,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FfStreamTags {
    pub language: Option<String>,
    pub title: Option<String>,
}
//ffprobe -v error -show_streams -print_format json -i $f
/// 获取视频文件的流数量
pub fn get_streams(video_file: &Path) -> AnyResult<FfStreamData> {
    let output = Command::new("ffprobe")
        .args(&[
            "-v",
            "error",
            "-show_streams",
            "-print_format",
            "json",
            "-i",
            &to_string(video_file),
        ])
        .output()?;

    if !output.status.success() {
        return Err(anyhow!(format!("ffprobe failed: {:?}", output)));
    }

    let stdout = String::from_utf8(output.stdout)?;
    let data: FfStreamData = json::from_str(&stdout)?;
    Ok(data)
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use path_macro::path;

    #[test]
    fn test_get_streams() {
        let f = path!("/mnt/temp/2025_07_21/huamu/C3_2025_07_21T12_53_33_L_100001.mkv");
        let stream_data = get_streams(&f).unwrap();

        println!("stream_data: {:#?}", stream_data);
        let gps_stream = stream_data.find_stream_by_title("GPS");
        println!("gps_stream: {:#?}", gps_stream);
        let gps_index = stream_data.stream_index_of_title("GPS");
        println!("gps_index: {:#?}", gps_index);
    }
}
