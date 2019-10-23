use rx::fs;
use std::path::{Path, PathBuf};

/// 媒体源ID
pub type SourceId = i32;

/// 组ID
pub type GroupId = i32;

/// 应用程序参数
pub struct AppParams {
    /// 项目ID
    project_id: String,

    /// 工作目录
    work_dir: PathBuf,

    /// 网点
    node: String,

    /// 配置名称，可切换成调试模式，测试模式，验证模式
    cfg_name: String,

    /// 模型名称
    model_name: String,

    /// 数据库名称
    db_name: String,

    /// 工作组ID
    group_id: GroupId,

    /// 显示详细信息
    verbose: bool,
}

const CFG: &str = "cfg";
const DB: &str = "db";
const MSG: &str = "msg";
const MODEL: &str = "model";
const SAMPLE: &str = "sample";
const RECORD: &str = "record";
const SNAPSHOT: &str = "snapshot";

impl AppParams {
    /// 获取配置路径
    pub fn cfg_dir(&self) -> PathBuf {
        self.work_dir.join(CFG).join(&self.cfg_name)
    }

    /// 获取模型路径
    pub fn model_dir(&self) -> PathBuf {
        self.work_dir.join(MODEL).join(&self.model_name)
    }

    /// 获取模型的标签文件路径	//TODO: 不兼容于ias
    pub fn label_file(&self, model: &str) -> PathBuf {
        self.model_dir().join(model).join("label.json")
    }

    /// 获取数据库路径
    pub fn db_dir(&self) -> PathBuf {
        self.dir_with_db(DB)
    }

    /// 获取报警数据路径
    pub fn msg_dir(&self) -> PathBuf {
        self.dir_with_db(MSG)
    }

    /// 获取样本路径
    pub fn sample_dir(&self) -> PathBuf {
        self.dir_with_db(SAMPLE)
    }

    /*
        /// 获取指定传感器样本路径
        pub fn sample_dir_of(&self, S sensor) -> PathBuf
        { self.sample_dir().join(sensor.id_dir() }
    */
    /// 获取录像路径
    pub fn record_dir(&self) -> PathBuf {
        self.dir_with_db(RECORD)
    }

    /// 获取设备录像路径
    pub fn record_dir_of(&self, source_id: SourceId) -> PathBuf {
        self.record_dir().join(source_id.to_string())
    }

    /// 获取截图路径
    pub fn snapshot_dir(&self) -> PathBuf {
        self.dir_with_db(SNAPSHOT)
    }

    /// 获取日志名称
    pub fn log_dir(&self) -> PathBuf {
        /*PathBuf p = dir_with_db("log");
        if (group_id > 0)
        p /= std::to_string(group_id);
        p;*/
        unimplemented!()
    }

    /// 获取节点话题
    pub fn node_topic(&self) -> String {
        self.project_id.clone() + "/nodes/" + &self.node
    }

    /// 检查组匹配
    pub fn match_group(&self, group: GroupId) -> bool {
        self.group_id < 1 || group == self.group_id
    }

    /// 获取组话题，组ID0表示所有组
    pub fn group_topic(&self) -> String {
        let g = if self.group_id > 0 {
            self.group_id.to_string()
        } else {
            "+".to_string()
        };
        self.node_topic() + "/groups/" + &g
    }

    /// 获取报警话题
    pub fn group_msg_topic(&self) -> String {
        self.group_topic() + "/" + MSG
    }

    /// 获取命令主题
    pub fn command_topic(&self) -> String {
        self.project_id.clone() + "/system/command"
    }

    /// 检查目录
    pub fn check_dir(&self) {
        unimplemented!()
    }

    /// 带数据库名的路径
    pub fn dir_with_db(&self, name: &str) -> PathBuf {
        self.work_dir
            .join(name)
            .join(&self.node)
            .join(&self.db_name)
    }
}
