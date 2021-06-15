use super::*;
use crate::basic::*;

/// 传感器基类
pub struct SensorBase {
    info: SensorInfo,
    cfg: SensorCfg,
    model_dir: PathBuf,
    //brief: String,
    rect: Rect,
}

impl SensorBase {
    /// 创建传感器
    pub fn _new(info: SensorInfo, cfg: SensorCfg, model_root: &Path) -> SensorBase {
        SensorBase {
            info,
            cfg,
            model_dir: model_root.to_path_buf(),
            rect: Rect::default(),
        }
    }

    pub fn info(&self) -> &SensorInfo {
        return &self.info;
    }

    pub fn cfg(&self) -> &SensorCfg {
        return &self.cfg;
    }

    //pub fn string brief()  ->
    //{ return brief; }

    /// 获取模型目录
    pub fn model_dir(&self) -> &Path {
        return self.model_dir.as_path();
    }

    /// 获取灵敏度
    pub fn sensitivity(&self) -> i32 {
        return self.info.params.sensitivity;
    }

    /// 获取触发阈值
    pub fn threshold(&self) -> i32 {
        return 100 - self.sensitivity();
    }

    /// 获取区域
    pub fn rect(&self) -> &Rect {
        return &self.rect;
    }
}
