use ias::app::*;
use rx::text::*;

fn main() {
    let app_info = AppInfo::new(
        "ias",
        "dump",
        "保存程序",
        "v0.1-alpha build 2019-10-23 16:49:01",
        "Howell J. <dayn9t@gmail.com>",
        "IAS dump service, dump alarm message into database",
    );

    let params = AppParams::parse_args(&app_info);

    println!("args: {}", to_json(&params).unwrap());
    println!("cfg: {:?}", params.cfg_dir());
    println!("log: {:?}", params.log_dir());
    println!("snapshot: {:?}", params.snapshot_dir());
}
