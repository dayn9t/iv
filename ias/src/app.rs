/// 构建信息
struct BuildInfo
{
    build_date: String,
    build_time: String,
}

///项目信息
struct ProjectInfo
{
    /// 项目ID
    id: String,

    /// 项目名称
    name: String,

    /// 相聚版本
    version: String,

    build_info: BuildInfo,

}

///应用程序信息
struct AppInfo
{
    id: String,
    /// 应用程序ID
    name: String,
    /// 应用程序名称
}