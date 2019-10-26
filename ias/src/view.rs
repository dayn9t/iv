use rx_core::text::*;
use rx_db::*;
use rx_net::mqtt::*;

use crate::alarm::*;
use crate::app::*;
//use crate::basic::*;
use crate::cfg::*;

/// 存储服务
pub struct DumpService {
    params: AppParams,
}

impl DumpService {
    /// 创建服务
    pub fn new(params: AppParams) -> DumpService {
        DumpService { params }
    }

    /// 加载配置，搜索顺序：1.节点配置, 2.公共配置
    pub fn load_cfg<T>(&self, cfg_name: &str) -> IoResult<T>
    where
        T: DeserializeOwned,
    {
        let cfg_name = cfg_name.to_owned() + ".json";
        let f = self.params.node_cfg_dir().join(&cfg_name);
        let mut cfg = load_json(f);

        if cfg.is_err() {
            let f = self.params.cfg_dir().join(&cfg_name);
            cfg = load_json(f);
        }
        cfg
    }

    /// 连接mqtt服务
    pub fn mqtt_connect(&self) -> MqttClient {
        let mqtt_cfg: MqttCfg = self.load_cfg("mqtt").unwrap();
        let client_id = self.params.app_full_name();
        println!("connect: {}", &mqtt_cfg.server_url);
        MqttClient::connect(&client_id, &mqtt_cfg.server_url).unwrap()
    }

    /// 启动服务
    pub fn run(&self) {
        let app_cfg: DumpCfg = self.load_cfg(&self.params.app_name()).unwrap();
        let mut db = RedisDb::open(&app_cfg.db_url).unwrap();
        let mut tab = db.open_table("alarm").unwrap();

        let mut client = self.mqtt_connect();
        let topic = self.params.group_topic();
        let rx = client.subscribe(&topic).unwrap();

        for m in rx {
            if let Some(m) = m {
                let info: AlarmInfo = serde_json::from_slice(m.payload()).unwrap();
                tab.put(info.id, &info).unwrap();
            }
        }
    }
}
