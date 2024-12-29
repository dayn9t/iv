use chrono::{Duration, NaiveTime};
use clap::Parser;
use rx_core::log::init_log;
use rx_core::prelude::*;
use rx_core::sys::fs::{SortOrder, files_in};
use rx_core::text::BoxResult;
use rx_core::time::ClockTime;
use std::path::PathBuf;

/// 命令行参数
#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
#[command(version, about, long_about = None)]
pub struct Opt {
    /// 文件所在目录
    #[arg(name = "DIR")]
    pub dir: PathBuf,

    /// 文件扩展名
    #[arg(name = "EXT")]
    pub ext: String,

    /// 开始时间
    #[arg(short, long)]
    pub start_time: NaiveTime,

    /// 间隔时间(ms)
    #[arg(short, long)]
    pub fps: f32,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

/*
   let url = "http://192.168.22.51:33721/tuling/asrc/v3/process";
   let src_file = "/iflytek/engine/lib/mkl/01.mp3";
   let dst_file = path!("/tmp/01.srt");
*/

fn main() -> BoxResult<()> {
    let opt = Opt::parse();
    init_log(opt.verbose as usize);

    let files = files_in(&opt.dir, &opt.ext, SortOrder::Asc)?;

    let start_time: NaiveTime = opt.start_time.into();
    let interval = (1000.0 / opt.fps) as i64;
    let interval = Duration::milliseconds(interval);

    for (i, file) in files.iter().enumerate() {
        let new_time = start_time + interval;
        let new_time = ClockTime::from(new_time);
        let new_name = format!(
            "{}-{}",
            new_time.to_str_id(),
            file.file_name().unwrap().to_str().unwrap()
        );
        let new_path = file.with_file_name(new_name);
        std::fs::rename(file, new_path)?;
    }
    println!("Done!");
    Ok(())
}
