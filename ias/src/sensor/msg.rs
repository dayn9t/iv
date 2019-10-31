use crate::basic::*;

/// 传感器状态
pub enum SensorState {
    /// 传感器正常
    Ok = 0,

    /// 传感器位置偏移
    Moved = 1,

    /// 传感器被遮挡
    Obscured = 2,

    /// 传感器读取失败
    Failed = 3,

    /// 传感器读取失败
    UnknownError = 99,
}

type SensorMsgId = i32;

type SensorId = i32;
type SourceId = i32;

/// 传感器消息
pub struct SensorMsg {
    /// 消息ID
    pub id: SensorMsgId,

    /// 传感器状态
    pub state: SensorState,

    /// 传感器上的读数(未来可能是多维数据)
    pub reading: f64,

    /// 发生时间
    pub time: NaiveDateTime,

    /// 截图URL
    pub image: String,

    /// 传感器信息
    pub from: SensorId,

    /// 视频源信息 ???
    pub source: SourceId,
    //inner: NetOutputInfo,
}

impl ToUuid for SensorMsg {
    /// 获取消息UUID
    fn to_uuid(&self) -> Uuid {
        unimplemented!()
    }
}

impl SensorMsg {
    /// 获取描述字符串
    pub fn _to_string(&self) -> String {
        unimplemented!()
    }

    /// 获取简要信息
    pub fn _brief(&self) -> String {
        unimplemented!()
    }
}

/// 传感器信息集合
pub type SensorMsgs = Vec<SensorMsg>;
