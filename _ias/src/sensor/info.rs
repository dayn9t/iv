use crate::basic::*;

/// 传感器参数信息
pub struct SensorParams {
    /// 型号
    pub model: String,

    /// 图片上的感兴趣区域
    pub roi: RoiD,

    /// 灵敏度
    pub sensitivity: i32,

    /// 取值范围
    pub range: RangeD,
}

type SensorId = i32;

type SourceId = i32;

/// 传感器信息
pub struct SensorInfo {
    /// 传感器ID
    pub id: SensorId,

    /// 传感器类型
    pub type_: i32, //TODO:

    /// 名称
    pub name: String,

    /// 传感器参数
    pub params: SensorParams,
    ///Calendar	CalendarTrigger	工作表目前没看出存在的必要	O

    /// 关联视频源Id
    pub source_id: SourceId,
    pub source_guid: Uuid,
    /// 允许状态
    pub enabled: bool,
}

impl SensorInfo {
    // 获取传感器类别
    //pub fn category(&self) -> SensorCategory {}

    // 获取传感器类别名称
    //pub fn category_name(&self) -> String {}

    // 获取类型名称
    //pub fn type_name(&self) -> String {}

    // 该传感器模型所在路径
    //pub fn model_dir(&self, model_root: PathBuf) -> PathBuf {}

    // 含有id的相对路径
    //pub fn id_dir(&self) -> PathBuf {}

    // 获取简要描述信息
    //pub fn brief(&self) -> String {}

    // 获取描述字符串
    //pub fn to_string(&self) -> String {}

    // 获取是否有效
    //pub fn valid(&self) -> bool {}

    // 设置节点信息
    //pub fn set(&self, node: NodeInfo) -> SensorInfo {}

    // 通过配置完善信息
    //pub fn complete_by(&self, cfg: SensorCfg) -> SensorInfo {}
}

/// 传感器信息集合
pub type SensorInfos = Vec<SensorInfo>;
