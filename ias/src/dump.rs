use rx_core::text::*;
use rx_db::*;
use rx_net::mqtt::*;

use crate::alarm::*;
use crate::app::*;
use crate::basic::*;

pub use std::io::Result as IoResult;

/// Mqtt配置信息
#[derive(Default, Clone, Serialize, Deserialize)]
struct MqttCfg {
    /// 传感器全局ID
    server_url: String,

    /// 根主题
    root_topic: String,
}

/// 存储配置信息
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct DumpCfg {
    /// 数据库URL
    db_url: String,
}

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

    /// 启动服务
    pub fn run(&self) {
        let mqtt_cfg: MqttCfg = self.load_cfg("mqtt").unwrap();
        let app_cfg: DumpCfg = self.load_cfg(&self.params.app_name()).unwrap();

        let topic = self.params.group_topic();
        let client_id = self.params.app_full_name();
        let mut client = MqttClient::connect(&client_id, &mqtt_cfg.server_url).unwrap();
        let mut rx = client.subscribe(&topic).unwrap();

        let mut db = RedisDb::open(&app_cfg.db_url).unwrap();
        let mut tab = db.open_table("alarm").unwrap();

        for m in rx {
            if let Some(m) = m {
                let info: AlarmInfo = serde_json::from_slice(m.payload()).unwrap();
                tab.put(info.id, &info).unwrap();
            }
        }
    }
}
