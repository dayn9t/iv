use crate::basic::*;

type SourceId = i32;

type GroupId = i32;

/// 视频源信息
struct SourceInfo
{
    /// 传感器ID
    id: SourceId,

    /// 名称
    name: String,

    /// 传感器链接
    url: String,

    /// 组Id
    group_id: GroupId,

    ///允许状态
    enabled: bool,

}