use rx::text::*;
use rx_db::*;
use rx_net::mqtt::*;

use crate::alarm::*;
use crate::basic::*;

///Mqtt配置信息
#[derive(Default, Clone, Serialize, Deserialize)]
struct MqttCfg {
    /// 传感器全局ID
    server_url: String,

    /// 根主题
    root_topic: String,
}

///存储配置信息
#[derive(Default, Clone, Serialize, Deserialize)]
struct DumpCfg {
    /// 传感器全局ID
    server_url: String,
}

pub fn do_dump() {
    let dir = Path::new("/home/maa/cfg/work/");
    let cfg: MqttCfg = load_json(dir.join("mqtt.json")).unwrap();
    let topic = cfg.root_topic + "/#";

    let mut client = MqttClient::connect("test_id", &cfg.server_url).unwrap();
    let mut rx = client.subscribe(&topic).unwrap();

    let cfg: DumpCfg = load_json(dir.join("dump.json")).unwrap();
    let mut db = RedisDb::open(&cfg.server_url).unwrap();
    let mut tab = db.open_table("alarm").unwrap();

    for m in rx {
        if let Some(m) = m {
            let info: AlarmInfo = serde_json::from_slice(m.payload()).unwrap();
            tab.put(info.id, &info);
        }
    }
}
