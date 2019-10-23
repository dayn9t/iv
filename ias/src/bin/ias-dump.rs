#[macro_use]
extern crate clap;

use ias::*;
/*
/// 报警来源信息
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct FromInfo {
    /// 规则ID
    rule_id: String,
    /// 报警设备ID
    device_id: DeviceId,
    /// 报警设备所在组ID
    group_id: GroupId,
    /// 所在网点ID
    node_id: NodeId,
}
*/

fn main() {
    let matches = clap_app!(dump =>
        (version: "0.1")
        (author: "J. <dayn9t@gmail.com>")
        (about: "IAS dump service, dump alarm message into db")
        (@arg v: -v --verbose "Print information verbosely")
        (@arg CONFIG: -c --config +takes_value "Sets the config name")
        (@arg DATABASE: -d --database +takes_value "Sets the database name")
        (@arg NODE: +required "Sets the node to use")
    )
    .get_matches();

    let node = matches.value_of("NODE").unwrap();
    println!("Using node: {}", node);

    let config = matches.value_of("CONFIG").unwrap_or("work");
    println!("Value for config: {}", config);

    let db = matches.value_of("DATABASE").unwrap_or("work");
    println!("Value for db: {}", db);

    let n = matches.occurrences_of("v") > 0;
    match n {
        false => println!("No verbose info"),
        true => println!("Some verbose info"),
    }
}
