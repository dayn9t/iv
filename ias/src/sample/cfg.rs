use crate::basic::*;
//use super::*;

/// PTZ信息
pub struct PtzInfo<T: iv_core::geo::ValidGeoType> {
    /// 平移
    pub pan: PointT<T>,

    /// 倾斜
    pub tilt: T,

    /// 缩放
    pub zoom: T,
}

pub type PtzInfoD = PtzInfo<f64>;

/// 改变与步长
pub struct DiffStepInfo {
    pub diff: f64,
    pub step: f64,
}

pub type PtzDiffStepInfo = PtzInfo<DiffStepInfo>;

/// 采样配置信息
pub struct SampleCfg {
    // 样本平移旋转缩放
//pub ptz: PtzDiffStepInfo,
}

/*
/// 用网格遍历PTZ参数空间
pub fn grid<F>(mut ptz: PtzDiffStepInfo, fun: F)
    where F: Fn(PtzInfoD)
{
    if ptz.pan.x.step == 0.0 { ptz.pan.x.step = ptz.pan.x.diff; }
    if ptz.pan.y.step == 0.0 { ptz.pan.y.step = ptz.pan.y.diff; }
    if ptz.tilt.step == 0.0 { ptz.tilt.step = ptz.tilt.diff; }
    if ptz.zoom.step == 0.0 { ptz.zoom.step = ptz.zoom.diff; }
    /*
            for (T dx = -ptz.pan.x.diff; dx <= ptz.pan.x.diff; dx += ptz.pan.x.step) {
                for (T dy = -ptz.pan.y.diff; dy <= ptz.pan.y.diff; dy += ptz.pan.y.step) {
                    fun({{dx, dy}, 0, 0});
    */
}*/
