use super::*;
use crate::basic::*;

/// 传感器基类
pub struct SensorBase {
    info: SensorInfo,
    cfg: SensorCfg,
    model_dir: PathBuf,
    //brief: String,
    //rect: Rect,
}

impl SensorBase {
    pub fn _new(info: SensorInfo, cfg: SensorCfg, model_root: &Path) -> SensorBase {
        SensorBase {
            info,
            cfg,
            model_dir: model_root.to_path_buf(),
        }
    }
    /*
    SensorInfo info()  override
    { return info; }

    SensorCfg cfg()  override
    { return cfg; }

    string brief()  override
    { return brief; }

    /// 获取模型目录
    Path model_dir()
    { return model_dir; }

    /// 获取灵敏度
    int sensitivity()
    { return info.params.sensitivity; }

    /// 获取触发阈值
    int threshold()
    { return 100 - sensitivity(); }

    /// 获取区域
    Rect rect()
    { return rect; }

    */
}
