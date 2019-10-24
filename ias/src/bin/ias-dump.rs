use ias::app::*;
use rx::text::*;

fn main() {
    let app_info = AppInfo::new(
        "dump",
        "IAS dump service, dump alarm message into database",
        ias::pkg(),
    );

    let params = AppParams::parse_args(&app_info);

    println!("args: {}", to_json(&params).unwrap());
    println!("cfg: {:?}", params.cfg_dir());
    println!("log: {:?}", params.log_dir());
    println!("snapshot: {:?}", params.snapshot_dir());
}
