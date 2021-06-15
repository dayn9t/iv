use crate::basic::*;

/// 网点ID
pub type NodeId = i32;

/// 节点信息
pub struct NodeInfo {
    /// 节点ID
    pub id: NodeId,

    /// 名称
    pub name: String,
    //
}

impl ToUuid for NodeInfo {
    /// 获取消息UUID
    fn to_uuid(&self) -> Uuid {
        unimplemented!()
    }
}
