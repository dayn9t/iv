mod processor;
mod subtitle;

use anyhow::Result;
use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "jsr-video-subs")]
#[command(about = "视频字幕合成工具", long_about = None)]
struct Args {
    /// 输入视频文件
    #[arg(short, long)]
    input: String,

    /// 输出视频文件
    #[arg(short, long)]
    output: String,

    /// 字幕文件列表（逗号分隔，由下往上排列）
    #[arg(short, long, value_delimiter = ',')]
    subs: Vec<String>,

    /// 字体大小（像素）
    #[arg(long, default_value = "24")]
    font_size: u32,

    /// 字体颜色
    #[arg(long, default_value = "white")]
    font_color: String,

    /// 字幕间距（默认=字体大小）
    #[arg(long)]
    spacing: Option<u32>,

    /// H.264 质量（0-51，越小越好）
    #[arg(long, default_value = "23")]
    crf: u32,

    /// 编码速度预设
    #[arg(long, default_value = "medium")]
    preset: String,

    /// 详细输出
    #[arg(short, long)]
    verbose: bool,
}

fn validate_args(args: &Args) -> Result<()> {
    // 检查输入文件存在
    if !Path::new(&args.input).exists() {
        return Err(anyhow::anyhow!("输入视频文件不存在: {}", args.input));
    }

    // 检查字幕文件存在
    for sub in &args.subs {
        if !Path::new(sub).exists() {
            return Err(anyhow::anyhow!("字幕文件不存在: {}", sub));
        }
    }

    // 检查 CRF 范围
    if args.crf > 51 {
        return Err(anyhow::anyhow!("CRF 值必须在 0-51 之间"));
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 验证参数
    validate_args(&args)?;

    if args.verbose {
        eprintln!("输入: {}", args.input);
        eprintln!("输出: {}", args.output);
        eprintln!("字幕: {:?}", args.subs);
        eprintln!("字体大小: {}", args.font_size);
        eprintln!("字体颜色: {}", args.font_color);
        eprintln!("间距: {:?}", args.spacing);
        eprintln!("CRF: {}", args.crf);
    }

    // 处理视频
    processor::process_video(
        Path::new(&args.input),
        Path::new(&args.output),
        &args.subs,
        args.font_size,
        &args.font_color,
        args.spacing,
        args.crf,
        &args.preset,
        args.verbose,
    )?;

    if args.verbose {
        eprintln!("✓ 处理完成");
    }

    Ok(())
}
