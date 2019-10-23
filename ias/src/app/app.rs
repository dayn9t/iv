/// 构建信息
pub struct BuildInfo {
    pub build_date: String,
    pub build_time: String,
}

/// 项目信息
pub struct ProjectInfo {
    /// 项目ID
    pub id: String,

    /// 项目名称
    pub name: String,

    /// 相聚版本
    pub version: String,

    pub build_info: BuildInfo,
}

/// 应用程序信息(项目包括多个应用程序)
pub struct AppInfo {
    /// 项目ID
    pub project: String,

    /// 应用程序ID
    pub id: String,

    /// 应用程序名称
    pub name: String,

    /// 应用程序版本
    pub version: String,

    /// 应用程序作者
    pub author: String,

    /// 应用程序表述信息
    pub about: String,
}

impl AppInfo {
    /// 创建引用程序信息
    pub fn new(
        project: &str,
        id: &str,
        name: &str,
        version: &str,
        author: &str,
        about: &str,
    ) -> AppInfo {
        AppInfo {
            project: project.to_owned(),
            id: id.to_owned(),
            name: name.to_owned(),
            version: version.to_owned(),
            author: author.to_owned(),
            about: about.to_owned(),
        }
    }

    /// 完整ID
    pub fn full_id(&self) -> String {
        self.project.clone() + "-" + &self.id
    }
}
