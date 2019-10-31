pub mod atm;
pub mod disaster;
pub mod elevator;
pub mod environment;
pub mod instrument;
pub mod vehicle;

/// 场景类型
pub enum SceneType {
    /// 车辆检测
    Vehicle = 1,

    /// 仪表读取
    Instrument = 2,

    /// 环境
    Environment = 3,

    /// 灾难
    Disaster = 4,

    /// 电梯内
    Elevator = 5,

    /// 银行ATM智能分析
    ATM = 6,
}
