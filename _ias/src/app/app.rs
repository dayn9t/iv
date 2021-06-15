use crate::basic::*;
use crate::cfg::*;
use rx_db::{RedisDb, RedisResult};
use rx_net::mqtt::*;

use super::params::*;

/// 应用程序
pub struct App {
    params: AppParams,
}

impl App {
    /// 创建服务
    pub fn new(params: AppParams) -> App {
        App { params }
    }

    /// 组主题
    pub fn group_topic(&self) -> String {
        self.params.group_topic()
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

    /// 加载应用程序配置
    pub fn load_app_cfg<T>(&self) -> IoResult<T>
    where
        T: DeserializeOwned,
    {
        self.load_cfg(&self.params.app_name())
    }

    /// 连接mqtt服务
    pub fn mqtt_connect(&self) -> MqttClient {
        let cfg: MqttCfg = self.load_cfg("mqtt").unwrap();
        let client_id = self.params.app_full_name();
        println!("connect mqtt server: {}", &cfg.server_url);
        MqttClient::connect(&client_id, &cfg.server_url).unwrap()
    }

    /// 打开热数据库
    pub fn open_hot_db(&self) -> RedisResult<RedisDb> {
        let cfg: DbCfg = self.load_cfg("db").unwrap();
        println!("connect redis server: {}", &cfg.hot_db);
        RedisDb::open(&cfg.hot_db)
    }
}
