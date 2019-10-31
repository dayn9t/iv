use std::time::SystemTime;

use crate::basic::*;

/// 报警类型
#[derive(Clone, Serialize, Deserialize)]
pub enum AlarmType {

    //通用报警

    /// 视频检查：信号丢失
    VideoLoss = 101,

    /// 视频检查：摄像机遮挡
    CameraBlocked = 102,

    /// 视频检查：摄像机移位
    CameraMoved = 103,

    /// 物品：遗留物
    Unattended = 121,

    /// 物品：移除
    Removal = 122,

    //定制报警：ATM机

    /// 插卡口异常
    AtmSlot = 401,

    /// 键盘异常
    AtmKeyboard = 402,

    /// 破坏ATM
    AtmDamage = 411,

    /// 脸部特征不清
    AtmFuzzyFace = 421,

    /// 脸部出现
    AtmFace = 422,

    //定制报警：防护舱
    /// 尾随进入
    CabinTailgating = 501,

    /// 强行推入
    CabinThrust = 502,

    //定制报警：加钞间

    /// 加钞间人数限制
    BackroomNumLimit = 601,

    /// 加钞间下蹲
    BackroomSquat = 611,

    //定制报警：自助银行大厅
    /// 徘徊
    HallLoitering = 701,

    /// 滞留
    HallRetention = 702,

    /// 打斗
    HallFighting = 711,

    /// 挟持
    HallSeizing = 712,

    //定制报警：无特定区域
    Damage = 801,
}

/// 规则Id
pub type RuleId = u32;

/// 设备Id
pub type DeviceId = u32;

/// 组Id
pub type GroupId = u32;

/// 节点Id
pub type NodeId = u32;

/// 报警Id
pub type AlarmId = usize;

/// 持续时间（秒）
pub type Duration = f64;

/// 报警来源信息
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct FromInfo {
    /// 规则ID
    rule_id: RuleId,
    /// 报警设备ID
    device_id: DeviceId,
    /// 报警设备所在组ID
    group_id: GroupId,
    /// 所在网点ID
    node_id: NodeId,
}

pub type Floats = Vec<f32>;

/// 内部保留信息
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct ReservedInfo {
    /// 分类器输出
    probs: Floats,
}

/// 报警信息
#[derive(Clone, Serialize, Deserialize)]
pub struct AlarmInfo {
    /// 报警事件ID
    pub id: AlarmId,

    /// 报警类型
    #[serde(rename = "type")]
    pub type_: AlarmType,

    /// 绝对时间
    pub time: UtcDateTime,

    /// 截图信息(部分图像可能不存在)
    pub images: Vec<String>,

    /// 录像文件
    pub record: String,

    /// 相对录像开始时间的相对时标
    pub timestamp: Duration,

    /// 置信度[0, 100]
    pub confidence: i32,

    /// 报警来源信息
    pub from: FromInfo,

    /// 内部保留信息，不需要呈现
    pub reserved: ReservedInfo,
}

impl AlarmInfo {
    /// 创建简要访问信息
    //pub fn new() -> Self {        AlarmInfo {}    }

    /// 获取描述字符串
    pub fn to_string(&self) -> String {
        String::new()
    }

    /// 获取描述字符串
    pub fn to_local_string(&self) -> String {
        String::new()
    }

    /// 替换路径前缀
    pub fn replace_path(&self, _src_prefix: &str, _dst_prefix: &str) -> AlarmInfo {
        unimplemented!()
    }
}
/*
impl Default for AlarmInfo {
    /// 创建简要访问信息
    fn default() -> Self {
        AlarmInfo {
            id: AlarmId::default(),
            type_: AlarmType::default(),
            time: UtcDateTime::from(SystemTime::now()),
            images: Vec::default(),
            record: String::default(),
            timestamp: Duration::default(),
            confidence: i32::default(),
            from: FromInfo::default(),
            reserved: ReservedInfo::default(),
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn var_works() {
        let a = AlarmType::AtmDamage;

        let _s = to_json(&a);

        //assert_eq!();
    }
}
