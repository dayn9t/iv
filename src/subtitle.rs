use anyhow::Result;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

/// 将 SRT 字幕文件转换为纯文本文件
/// FFmpeg drawtext 需要纯文本，去除序号和时间轴
pub fn convert_srt_to_text(srt_path: &Path, txt_path: &Path) -> Result<()> {
    let file = File::open(srt_path)?;
    let reader = BufReader::new(file);
    let mut out = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(txt_path)?;

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        // 跳过空行
        if trimmed.is_empty() {
            continue;
        }

        // 跳过序号行（纯数字）
        if trimmed.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }

        // 跳过时间轴行
        if trimmed.contains("-->") {
            continue;
        }

        // 写入文本行
        writeln!(out, "{}", trimmed)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_convert_srt_to_text() {
        let temp_dir = TempDir::new().unwrap();
        let srt_path = temp_dir.path().join("test.srt");
        let txt_path = temp_dir.path().join("test.txt");

        // 创建测试 SRT 文件
        let srt_content = r#"1
00:00:00,000 --> 00:00:02,000
第一行字幕

2
00:00:02,000 --> 00:00:04,000
第二行字幕
"#;

        fs::write(&srt_path, srt_content).unwrap();

        // 转换
        convert_srt_to_text(&srt_path, &txt_path).unwrap();

        // 验证结果
        let result = fs::read_to_string(&txt_path).unwrap();
        assert_eq!(result.trim(), "第一行字幕\n第二行字幕");
    }
}
