use rx_db::*;

use crate::alarm::*;
use crate::app::*;
//use crate::basic::*;
use crate::cfg::*;

/// 存储服务
pub struct DumpService {
    app: App,
}

impl DumpService {
    /// 创建服务
    pub fn new(params: AppParams) -> DumpService {
        let app = App::new(params);
        DumpService { app }
    }

    /// 启动服务
    pub fn run(&self) {
        let _app_cfg: DumpCfg = self.app.load_app_cfg().unwrap();

        let mut db = self.app.open_hot_db().unwrap();
        let mut tab = db.open_table("msg").unwrap();

        let mut client = self.app.mqtt_connect();
        let topic = self.app.group_topic(); //TODO
        let rx = client.subscribe(&topic).unwrap();

        for m in rx {
            if let Some(m) = m {
                let info: AlarmInfo = serde_json::from_slice(m.payload()).unwrap();
                tab.put(info.id, &info).unwrap();
            }
        }
        //TODO 尝试合并maa & ias
    }
}
