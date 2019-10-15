use crate::basic::*;

const I: i32 = 5;

struct AT(i32);

const C: AT = AT(0);

/// 报警类型
#[repr(u32)]
#[derive(Clone, Serialize, Deserialize)]
enum _AlarmType {
    NONE = 0,

    //通用报警
    /// 视频检查：信号丢失
    VIDEO_LOSS = 101,
    /// 视频检查：摄像机遮挡
    CAMERA_BLOCKED = 102,
    /// 视频检查：摄像机移位
    CAMERA_MOVED = 103,
    /// 物品：遗留物
    UNATTENDED = 121,
    /// 物品：移除
    REMOVAL = 122,

    //定制报警：ATM机
    /// 插卡口异常
    ATM_SLOT = 401,
    /// 键盘异常
    ATM_KEYBOARD = 402,
    /// 破坏ATM
    ATM_DAMAGE = 411,
    /// 脸部特征不清
    ATM_FUZZY_FACE = 421,
    /// 脸部出现
    ATM_FACE = 422,

    //定制报警：防护舱
    /// 尾随进入
    CABIN_TAILGATING = 501,
    /// 强行推入
    CABIN_THRUST = 502,

    //定制报警：加钞间
    /// 加钞间人数限制
    BACKROOM_NUM_LIMIT = 601,
    /// 加钞间下蹲
    BACKROOM_SQUAT = 611,

    //定制报警：自助银行大厅
    /// 徘徊
    HALL_LOITERING = 701,
    /// 滞留
    HALL_RETENTION = 702,
    /// 打斗
    HALL_FIGHTING = 711,
    /// 挟持
    HALL_SEIZING = 712,

    //定制报警：无特定区域
    DAMAGE = 801,
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
