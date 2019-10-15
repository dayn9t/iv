use ias::*;
use rx::text::*;

fn main() {
    let a: AlarmInfo = load_json("/home/jiang/rs/iv/ias/data/alarm.json").unwrap();

    let s = to_json(&a).unwrap();

    println!("Json: {}", s);
    //let
}
