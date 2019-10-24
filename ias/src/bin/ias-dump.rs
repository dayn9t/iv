use ias::app::*;
use rx::text::*;

fn main() {
    println!("VERGEN_SEMVER: {}", env!("VERGEN_SEMVER"));
    println!("VERGEN_SHA_SHORT: {}", env!("VERGEN_SHA_SHORT"));
    println!("VERGEN_COMMIT_DATE: {}", env!("VERGEN_COMMIT_DATE"));
    println!("VERGEN_SHA: {}", env!("VERGEN_SHA"));
    println!("CARGO_PKG_VERSION: {}", env!("CARGO_PKG_VERSION"));

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
