use ias::*;
use rx::text::*;

fn main() {
    let a: AlarmInfo = load_json("/home/jiang/rs/iv/ias/data/alarm.json").unwrap();

    let dt1 = a.time.naive_local();
    let dt2 = a.time.with_timezone(&ias::Local).naive_local();
    let dt3 = a.time.with_timezone(&ias::Local).naive_utc();
    println!("T: {}", dt1);
    println!("T: {}", dt2);
    println!("T: {}", dt3);

    let s = to_json(&a).unwrap();

    println!("Json: {}", s);
    //let
}