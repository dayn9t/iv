use crate::basic::*;

/// 报警类型
pub mod alarm_type {
    pub const NONE: i32 = 0;

    //通用报警

    /// 视频检查：信号丢失
    pub const VIDEO_LOSS: i32 = 101;

    /// 视频检查：摄像机遮挡
    pub const CAMERA_BLOCKED: i32 = 102;

    /// 视频检查：摄像机移位
    pub const CAMERA_MOVED: i32 = 103;

    /// 物品：遗留物
    pub const UNATTENDED: i32 = 121;

    /// 物品：移除
    pub const REMOVAL: i32 = 122;

    //定制报警：ATM机

    /// 插卡口异常
    pub const ATM_SLOT: i32 = 401;

    /// 键盘异常
    pub const ATM_KEYBOARD: i32 = 402;

    /// 破坏ATM
    pub const ATM_DAMAGE: i32 = 411;

    /// 脸部特征不清
    pub const ATM_FUZZY_FACE: i32 = 421;

    /// 脸部出现
    pub const ATM_FACE: i32 = 422;

    //定制报警：防护舱
    /// 尾随进入
    pub const CABIN_TAILGATING: i32 = 501;

    /// 强行推入
    pub const CABIN_THRUST: i32 = 502;

    //定制报警：加钞间

    /// 加钞间人数限制
    pub const BACKROOM_NUM_LIMIT: i32 = 601;

    /// 加钞间下蹲
    pub const BACKROOM_SQUAT: i32 = 611;

    //定制报警：自助银行大厅
    /// 徘徊
    pub const HALL_LOITERING: i32 = 701;

    /// 滞留
    pub const HALL_RETENTION: i32 = 702;

    /// 打斗
    pub const HALL_FIGHTING: i32 = 711;

    /// 挟持
    pub const HALL_SEIZING: i32 = 712;

    //定制报警：无特定区域
    pub const DAMAGE: i32 = 801;
}

/// 规则Id
type RuleId = u32;

/// 设备Id
type DeviceId = u32;

/// 组Id
type GroupId = u32;

/// 节点Id
type NodeId = u32;

/// 报警类型
pub type AlarmType = i32;

/// 报警Id
pub type AlarmId = i32;

/// 持续时间（秒）
pub type Duration = f64;

/// 报警来源信息
#[derive(Default, Clone, Serialize, Deserialize)]
struct FromInfo {
    /// 规则ID
    rule_id: RuleId,
    /// 报警设备ID
    device_id: DeviceId,
    /// 报警设备所在组ID
    group_id: GroupId,
    /// 所在网点ID
    node_id: NodeId,
}

type floats = Vec<f32>;

/// 内部保留信息
#[derive(Default, Clone, Serialize, Deserialize)]
struct ReservedInfo {
    /// 分类器输出
    probs: floats,
}

/// 报警信息
#[derive(Clone, Serialize, Deserialize)]
pub struct AlarmInfo {
    /// 报警事件ID
    id: AlarmId,

    /// 报警类型
    #[serde(rename = "type")]
    type_: AlarmType,

    /// 绝对时间，格式:"2011-11-01 13:45:23.120"
    pub time: UtcDateTime,

    /// 截图信息(部分图像可能不存在)
    images: Vec<String>,

    /// 关注区域图标
    ico_file: String,

    /// 录像文件
    record: String,

    /// 相对录像开始时间的相对时标
    timestamp: Duration,

    /// 置信度[0, 100]
    confidence: i32,

    /// 报警来源信息
    from: FromInfo,

    /// 内部保留信息，不需要呈现
    reserved: ReservedInfo,
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
    pub fn replace_path(&self, src_prefix: &str, dst_prefix: &str) -> AlarmInfo {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn var_works() {
        let a = alarm_type::ATM_DAMAGE;

        let s = to_json(&a);

        //assert_eq!();
    }
}
