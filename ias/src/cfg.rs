use crate::basic::*;

/// Mqtt配置信息
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct MqttCfg {
    /// 传感器全局ID
    pub server_url: String,

    /// 根主题
    pub root_topic: String,
}

/// 存储配置信息
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct DumpCfg {
    /// 数据库URL
    pub db_url: String,
}
