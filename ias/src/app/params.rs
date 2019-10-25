use std::path::PathBuf;

//use serde::{Deserialize, Serialize};
//use rx_core::fs;
use rx_core::text::*;

use super::app::AppInfo;

/// 媒体源ID
pub type SourceId = i32;

/// 组ID
pub type GroupId = i32;

/// 应用程序参数
#[derive(Deserialize, Serialize)]
pub struct AppParams {
    /// 包名称
    package: String,

    /// 应用程序ID
    app: String,

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
    verbose: u64,
}

const CFG: &str = "cfg";
const DB: &str = "db";
const MSG: &str = "msg";
const MODEL: &str = "model";
const LOG: &str = "log";
const SAMPLE: &str = "sample";
const RECORD: &str = "record";
const SNAPSHOT: &str = "snapshot";

impl AppParams {
    /// 创建命令行参数
    pub fn new(
        app_info: &AppInfo,
        node: &str,
        cfg_name: &str,
        model_name: &str,
        db_name: &str,
        group_id: GroupId,
        verbose: u64,
    ) -> AppParams {
        let package = app_info.package.name.to_owned();
        AppParams {
            package: package.clone(),
            app: app_info.name.clone(),
            work_dir: PathBuf::from("/var/").join(package),
            node: node.to_owned(),
            cfg_name: cfg_name.to_owned(),
            model_name: model_name.to_owned(),
            db_name: db_name.to_owned(),
            group_id,
            verbose,
        }
    }

    /// 解析命令行参数
    pub fn parse_args(app_info: &AppInfo) -> AppParams {
        use clap::{App, Arg};

        let matches = App::new(app_info.full_name())
            .version(app_info.package.full_version().as_str())
            .author(app_info.package.authors)
            .about(app_info.about.as_str())
            .arg(
                Arg::with_name("CONFIG")
                    .short("c")
                    .long("config")
                    .default_value("work")
                    .help("Sets the config name")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("DATABASE")
                    .short("d")
                    .long("database")
                    .default_value("work")
                    .help("Sets the database name")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("MODEL")
                    .short("m")
                    .long("model")
                    .default_value("work")
                    .help("Sets the model name")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("GROUP")
                    .short("g")
                    .long("group")
                    .default_value("0")
                    .help("Sets the group number")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("NODE")
                    .help("Sets the node to use")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::with_name("v")
                    .short("v")
                    .multiple(true)
                    .help("Sets the level of verbosity"),
            )
            .get_matches();

        let node = matches.value_of("NODE").unwrap();
        let cfg_name = matches.value_of("CONFIG").unwrap();
        let model_name = matches.value_of("MODEL").unwrap();
        let db_name = matches.value_of("DATABASE").unwrap();
        let group_id = matches.value_of("GROUP").unwrap().parse().unwrap();
        let verbose = matches.occurrences_of("v");

        AppParams::new(
            app_info, node, cfg_name, model_name, db_name, group_id, verbose,
        )
    }

    /// 获取配置路径
    pub fn cfg_dir(&self) -> PathBuf {
        self.work_dir.join(CFG).join(&self.cfg_name)
    }

    /// 获取模型路径
    pub fn model_dir(&self) -> PathBuf {
        self.work_dir.join(MODEL).join(&self.model_name)
    }

    /// 获取模型的标签文件路径	//TODO: 不兼容于ias
    pub fn label_file(&self, catalog: &str, model: &str) -> PathBuf {
        self.model_dir()
            .join(catalog)
            .join(model)
            .join("label.json")
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
        let mut p = self.dir_with_db(LOG);
        let mut app_id = self.app.clone();
        if self.group_id > 0 {
            app_id += &self.group_id.to_string();
        }
        p.push(app_id);
        p
    }

    /// 获取节点话题
    pub fn node_topic(&self) -> String {
        self.package.clone() + "/nodes/" + &self.node
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
        self.package.clone() + "/system/command"
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn param_works() {
        let app_info = AppInfo::new("app", "IAS test service", crate::pkg());

        const G: GroupId = 9;
        const V: u64 = 3;

        let p = AppParams::new(&app_info, "node1", "cfg1", "mod1", "db1", G, V);

        assert_eq!(p.cfg_dir(), PathBuf::from("/var/ias/cfg/cfg1"));
        assert_eq!(p.db_dir(), PathBuf::from("/var/ias/db/node1/db1"));
        assert_eq!(p.model_dir(), PathBuf::from("/var/ias/model/mod1"));
        assert_eq!(
            p.snapshot_dir(),
            PathBuf::from("/var/ias/snapshot/node1/db1")
        );
        assert_eq!(p.record_dir(), PathBuf::from("/var/ias/record/node1/db1"));
        assert_eq!(p.msg_dir(), PathBuf::from("/var/ias/msg/node1/db1"));
        assert_eq!(p.sample_dir(), PathBuf::from("/var/ias/sample/node1/db1"));
        assert_eq!(p.log_dir(), PathBuf::from("/var/ias/log/node1/db1/app9"));
        assert_eq!(
            p.label_file("vehicle", "front"),
            PathBuf::from("/var/ias/model/mod1/vehicle/front/label.json")
        );
        assert_eq!(p.group_topic(), "ias/nodes/node1/groups/9");
        assert_eq!(p.group_msg_topic(), "ias/nodes/node1/groups/9/msg");
    }
}
