// mod atm;
// mod disaster;
// mod elevator;
// mod environment;
// mod instrument;
// mod vehicle;

/// 传感器类型
pub enum SensorType {
    /// 安培表
    Ampere = 211,

    /// 伏特表
    Voltmeter = 212,

    /// 温度计
    Thermometer = 221,

    /// 液位表
    LevelGauge = 231,
}
