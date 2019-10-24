use ias::app::*;
use rx::text::*;

fn main() {

    println!("VERGEN_SEMVER: {}", env!("VERGEN_SEMVER"));
    println!("VERGEN_SHA_SHORT: {}", env!("VERGEN_SHA_SHORT"));
    println!("VERGEN_COMMIT_DATE: {}", env!("VERGEN_COMMIT_DATE"));
    println!("VERGEN_SHA: {}", env!("VERGEN_SHA"));

    let app_info = AppInfo::new(
        "ias",
        "dump",
        "保存程序",
        env!("VERGEN_BUILD_TIMESTAMP"),
        "Howell J. <dayn9t@gmail.com>",
        "IAS dump service, dump alarm message into database",
    );

    let params = AppParams::parse_args(&app_info);

    println!("args: {}", to_json(&params).unwrap());
    println!("cfg: {:?}", params.cfg_dir());
    println!("log: {:?}", params.log_dir());
    println!("snapshot: {:?}", params.snapshot_dir());


}
