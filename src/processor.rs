use anyhow::Result;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

/// 处理视频，将字幕燃烧到画面中
pub fn process_video(
    input: &Path,
    output: &Path,
    subs: &[String],
    font_size: u32,
    font_color: &str,
    spacing: Option<u32>,
    crf: u32,
    preset: &str,
    verbose: bool,
) -> Result<()> {
    let spacing = spacing.unwrap_or(font_size);
    let temp_dir = TempDir::new()?;

    // 将所有 SRT 转换为临时文本文件
    let mut text_files = Vec::new();
    for (i, sub_path) in subs.iter().enumerate() {
        let txt_path = temp_dir.path().join(format!("sub_{}.txt", i));
        crate::subtitle::convert_srt_to_text(Path::new(sub_path), &txt_path)?;
        text_files.push(txt_path);
    }

    // 生成 FFmpeg 滤镜链
    let mut filters = Vec::new();
    for (i, txt_path) in text_files.iter().enumerate() {
        let y_pos = format!("(H-{font_size})-{i}*({font_size}+{spacing})");
        let filter = format!(
            "drawtext=textfile='{}':fontsize={}:fontcolor={}:x=(W-tw)/2:y={}:fix_bounds=true",
            txt_path.display(),
            font_size,
            font_color,
            y_pos
        );
        filters.push(filter);
    }

    let filter_complex = filters.join(",");

    if verbose {
        eprintln!("滤镜链: {}", filter_complex);
    }

    // 调用 FFmpeg
    let output_ffmpeg = Command::new("ffmpeg")
        .arg("-y")  // 覆盖输出文件
        .arg("-i")
        .arg(input)
        .arg("-vf")
        .arg(&filter_complex)
        .arg("-c:v")
        .arg("libx264")
        .arg("-crf")
        .arg(crf.to_string())
        .arg("-preset")
        .arg(preset)
        .arg("-c:a")
        .arg("aac")  // 音频编码为 AAC
        .arg(output)
        .output()?;

    if verbose {
        eprintln!("FFmpeg stderr: {}", String::from_utf8_lossy(&output_ffmpeg.stderr));
    }

    if !output_ffmpeg.status.success() {
        return Err(anyhow::anyhow!(
            "FFmpeg 失败: {}",
            String::from_utf8_lossy(&output_ffmpeg.stderr)
        ));
    }

    Ok(())
}
