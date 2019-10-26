use rx_core::text::*;

use ias::app::*;
use ias::*;

fn main() {
    let app_info = AppInfo::new(
        "view",
        "IAS view service, view alarm message",
        ias::package_info(),
    );

    let params = AppParams::parse_args(&app_info);

    println!("node_topic: {}", params.node_topic());
    println!("command_topic: {}", params.command_topic());
    println!("group_msg_topic: {}", params.group_msg_topic());
    println!("group_topic: {}", params.group_topic());

    let a: AlarmInfo = load_json("/home/jiang/rs/iv/ias/data/maa-alarm.json").unwrap();
    let s = to_json(&a).unwrap();

    println!("Json: {}", s);
}
