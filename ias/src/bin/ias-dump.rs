use ias::*;
use rx::text::*;
use rx_net::mqtt::*;

fn main() {

    let topic = "test";
    let payload = "hi111你好吗";

    let mut client = MqttClient::connect("test_id", "tcp://localhost:1883").unwrap();
    let mut rx = client.subscribe(topic).unwrap();
    client.publish(topic, payload).unwrap();

    let m = rx.iter().next().unwrap().unwrap();

    assert_eq!(m.topic(), topic);
    assert_eq!(m.payload(), payload.as_bytes());

    let a = alarm_type::ATM_DAMAGE;

    let s = to_json(&a).unwrap();
    println!("Json: {}", s);

    let a: AlarmInfo = load_json("/home/jiang/rs/iv/ias/data/alarm.json").unwrap();

    let s = to_json(&a).unwrap();

    println!("Json: {}", s);
    //let
}