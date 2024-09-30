use std::{any::Any, collections::HashMap};

use salvo::{Depot, Request, Response};
use serde::Serialize;

//==================================================================================
//= 插件需要实现的接口
//= interface of plugin
pub trait Plugin: Any + Send + Sync {
    fn name(&self) -> &'static str;

    // fn on_plugin_load(&self) {}
    // /// you need to do any cleanup.
    // fn on_plugin_unload(&self) {}
    // /// Inspect (and possibly mutate) the request before it is sent.
    // fn pre_send(&self, _request: &mut Request) {}
    // /// Inspect and/or mutate the received response before it is displayed to
    // /// the user.
    // fn post_receive(&self, _response: &mut Response) {}

    // fn plugin_post(&self, req: &mut Request, depot: &mut Depot, res: &mut Response);
    // fn plugin_delete(&self, req: &mut Request, depot: &mut Depot, res: &mut Response);
    // fn plugin_put(&self, req: &mut Request, depot: &mut Depot, res: &mut Response);
    fn plugin_get(&self, req: &mut Request, depot: &mut Depot, res: &mut Response) -> CommonData;
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct CommonData {
    pub data: Option<HashMap<String, String>>,
    pub double_data: Option<HashMap<String, HashMap<String, String>>>,
}

impl CommonData {
    pub fn new() -> Self {
        Self {
            data: Some(HashMap::new()),
            double_data: Some(HashMap::new()),
        }
    }
}

// impl Clone for CommonData {
//     fn clone(&self) -> Self {
//         Self {
//             data: self.data.clone(),
//             double_data: self.double_data.clone(),
//         }
//     }
// }
