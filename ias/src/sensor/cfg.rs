use crate::basic::*;
use crate::sample::SampleCfg;

use super::*;

type Floats = Vec<f32>;

/// 网络内部信息信息
pub struct NetOutputInfo {
    /// 网络输出
    pub output: Floats,

    /// 触发区域，可为空
    pub rect: Rect,

    /// 发现移动
    pub moved: bool,

    /// 读数发生变化
    pub changed: bool,
}

impl NetOutputInfo {
    /// 更新了
    pub fn updated(&self) -> bool {
        return self.moved || self.changed;
    }

    /// 初始化
    pub fn init(&mut self, _moved: bool) {
        self.moved = _moved;
        self.changed = false;
        self.rect = Rect::default();
    }

    /// 设置默认为分类
    pub fn set_default_class(&mut self, class_index: usize) {
        self.output.clear();
        self.output.resize(class_index + 1, 0.0);
        self.output[class_index] = 0.1;
    }

    /// 设置分类器输出
    pub fn set_output(&mut self, _output: Floats, _rect: Rect) {
        self.changed = self.output != _output;
        self.output = _output;
        self.rect = _rect;
    }

    /// 比较最后一个分类的概率，概率大在前
    pub fn lt(&self, info: &NetOutputInfo) -> bool {
        assert!(!self.output.is_empty());
        assert_eq!(info.output.len(), self.output.len());
        return self.output.last() > info.output.last();
    }

    /// 获取简要信息
    pub fn brief(&self) -> String {
        unimplemented!()
    }
}

///运动优化网络配置
pub struct MdoNetCfg {
    /// 变化率阈值
    pub motion_ratio_thr: f64,

    /// 最大跳过DL判定次数
    pub max_skip_times: i32,

    /// 像素变化阈值
    pub pixel_change_thr: i32,
}

/// 前景块网络配置
pub struct FgbNetCfg {
    /// 背景模型学习速率
    pub learning_rate: f64,

    /// 块变化率阈值
    pub change_thr: f64,

    /// 分块尺寸
    pub block: Size,

    /// 最大输出结果
    pub max_output: i32,
}

/// 回归网络配置
pub struct RegNetCfg {
    /// 背景模型学习速率
    pub learning_rate: f64,

    /// 块变化率阈值
    pub change_thr: f64,

    /// 分块尺寸
    pub block: Size,

    /// 最大输出结果
    pub max_output: i32,
}

/// 模型配置
pub enum NetCfg {
    /// 运动检测优化网络配置
    Mdo(MdoNetCfg),

    /// 前景块网络配置
    Fgb(FgbNetCfg),

    /// 回归网络配置
    Reg(RegNetCfg),
}

/// 模型配置
pub struct ModelCfg {
    /// 模型名称
    pub name: String,

    /// 分析模型所需图像尺寸
    pub size: Size,

    /// 默认类别
    pub default_class: i32,

    /// 分类得分表
    pub score_tab: Vec<i32>,

    /// 回归网络配置
    pub net_cfg: NetCfg,
}

/// 传感器配置
pub struct SensorCfg {
    /// 分析所需输入图像尺寸
    pub size: Size,

    /// 规则默认参数
    pub params: SensorParams,

    /// 保留标志
    pub flags: i32,

    //pub Duration report_interval,    /// 报警最小间隔
    /// 边长延长相对于标定数据的比率
    pub side_ext: f64,

    /// 采样配置信息
    pub sample: SampleCfg,

    /// 模型配置
    pub model: ModelCfg,
}
/*
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

type SensorId = i32,
*/

//type SourceId = i32;

/*



/// 传感器类型配置表
using SensorCfgMap = std::map < SensorCategory, SensorCfg >,


/// 传感器Id
using SensorMsgId = cx::StrongId <int64_t, SensorMsg >;
*/
