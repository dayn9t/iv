//use rx_db::*;

//use crate::alarm::*;
use crate::app::*;
use crate::cfg::*;

/// 存储服务
pub struct ViewService {
    app: App,
}

impl ViewService {
    /// 创建服务
    pub fn new(params: AppParams) -> ViewService {
        let app = App::new(params);
        ViewService { app }
    }

    /// 启动服务
    pub fn run(&self) {
        let _cfg: DumpCfg = self.app.load_app_cfg().unwrap();
        //let mut db = RedisDb::open(&app_cfg.db_url).unwrap();
        //let mut tab = db.open_table("alarm").unwrap();
        //let _msg: AlarmInfo = tab.get(1).unwrap();
    }
}
