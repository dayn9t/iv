use ias::app::*;
use ias::*;
use rx::text::*;

fn main() {
    let app_info = AppInfo::new(
        "dump",
        "IAS dump service, dump alarm message into database",
        ias::pkg(),
    );

    let params = AppParams::parse_args(&app_info);

    println!("node_topic: {}", params.node_topic());
    println!("command_topic: {}", params.command_topic());
    println!("group_msg_topic: {}", params.group_msg_topic());
    println!("group_topic: {}", params.group_topic());

    let a: AlarmInfo = load_json("/home/jiang/rs/iv-core/ias/data/maa-alarm.json").unwrap();
    let s = to_json(&a).unwrap();

    println!("Json: {}", s);
}
