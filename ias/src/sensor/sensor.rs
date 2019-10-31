/*

using iv-core::NetOutputInfo;

using iv-core::NetOutputInfoSet;

using iv-core::TraggerState;

/// 浮点型感兴趣区域
using RoiD = iv-core::PointDs;
*/





/// 传感器状态,TODO: 序列化时改成整数值
pub enum State
{
    /// 传感器正常
    ok = 0,

    /// 传感器位置偏移
    moved = 1,

    /// 传感器被遮挡
    occlusion = 2,

    /// 传感器读取失败
    fail = 3,

    /// 传感器读取失败
    unknown_error = 99,
}


/// 传感器参数信息
struct SensorParams
{
    /// 型号
    model: String,
    /// 图片上的感兴趣区域
    roi: RoiD,
    /// 灵敏度
    sensitivity: i32,
    /// 取值范围
    range: RangeD,
}

type SensorId = i32;


type SourceId = i32;

/// 传感器信息
struct SensorInfo
{
    /// 传感器ID
    id: SensorId,
    /// 传感器全局ID
    guid: Uuid,
    /// 传感器类型
    type_: SensorType,
    /// 名称
    name: String,
    /// 传感器参数
    params: SensorParams,
    ///Calendar	CalendarTrigger	工作表目前没看出存在的必要	O

    /// 关联视频源Id
    source_id: SourceId,
    source_guid: Uuid,
    /// 允许状态
    enabled: bool,
}

impl SensorInfo {
    /// 获取传感器类别
    pub fn category(&self) -> SensorCategory {}

    /// 获取传感器类别名称
    pub fn category_name(&self) -> String {}

    /// 获取类型名称
    pub fn type_name(&self) -> String {}

    /// 该传感器模型所在路径
    pub fn model_dir(&self, model_root: PathBuf) -> PathBuf {}

    /// 含有id的相对路径
    pub fn id_dir(&self) -> PathBuf {}

    /// 获取简要描述信息
    pub fn brief(&self) -> String {}

    /// 获取描述字符串
    pub fn to_string(&self) -> String {}

    /// 获取是否有效
    pub fn valid(&self) -> bool {}

    /// 设置节点信息
    pub fn set(&self, node: NodeInfo) -> SensorInfo {}

    /// 通过配置完善信息
    pub fn complete_by(&self, cfg: SensorCfg) -> SensorInfo {}
}


/// 传感器信息集合
type SensorInfos = Vec<SensorInfo>;

/*
/// 传感器配置
struct SensorCfg
{
    Size size,                     /// 分析所需输入图像尺寸
    SensorParams params,           /// 规则默认参数
    int flags = 0,                 /// 保留标志
//Duration report_interval,    /// 报警最小间隔
    double side_ext = 0,           /// 边长延长相对于标定数据的比率
    iv-core::SampleCfg sample,           /// 采样配置信息
    iv-core::ModelCfg model,         /// 模型配置
}


/// 传感器类型配置表
using SensorCfgMap = std::map < SensorCategory, SensorCfg >,


/// 传感器Id
using SensorMsgId = cx::StrongId <int64_t, SensorMsg >;
*/

type SensorMsgId = i32;

/// 传感器消息信息
struct SensorMsg
{
    /// 消息ID
    id: SensorMsgId,

    /// 消息全局ID
    guid: Uuid,

    /// 传感器状态
    state: SensorState,

    /// 传感器上的读数
    reading: f64,

    /// 发生时间
    time: TimePoint,

    /// 截图URL
    image: String,

    /// 传感器信息
    sensor: SensorInfo,

    /// 视频源信息
    source: SourceInfo,
    inner: NetOutputInfo,
}

impl SensorMsg {
    /// 获取描述字符串
    pub fn to_string(&self) -> String {}

    /// 获取简要信息
    pub fn brief(&self) -> String {}

    /// 补全信息
    pub fn set(&self, node: NodeInfo)
    {}
}


/// 传感器信息集合
type SensorMsgs = vector<SensorMsg>;

/// 传感器区域正方形扩展
pub fn square_input_rect(info: SensorInfo, cfg: SensorCfg) -> Rect {}

/// 传感器区域长方形扩展
pub fn rectangle_input_rect(info: SensorInfo, cfg: SensorCfg) -> Rect {}

/// 产品（型号）信息
struct ProductInfo
{
    degree_to_scale: doubles,
}

