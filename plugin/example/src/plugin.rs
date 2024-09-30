use std::collections::HashMap;

use interface::plugin::{CommonData, Plugin};

//==================================================================================
//= 导出结构体的函数
//= Function deriving structure
#[no_mangle]
pub fn create_plugin() -> *mut dyn Plugin {
    Box::into_raw(Box::new(Example::new()))
}

#[derive(Clone)]
pub struct Example;

impl Example {
    fn new() -> Self {
        Example
    }
}

//==================================================================================
//= 实现插件接口
//= Implement plugin interface
impl Plugin for Example {
    fn name(&self) -> &'static str {
        "Example"
    }

    fn plugin_get(
        &self,
        _req: &mut salvo::Request,
        _depot: &mut salvo::Depot,
        _res: &mut salvo::Response,
    ) -> CommonData {
        CommonData {
            data: Some(HashMap::from([("key".to_string(), "value".to_string())])),
            double_data: Some(HashMap::from([(
                "key1".to_string(),
                HashMap::from([("key2".to_string(), "value2".to_string())]),
            )])),
        }
    }
}

//==================================================================================
//= 插件析构
//= Plugin destructor
impl Drop for Example {
    fn drop(&mut self) {
        println!("Dropping Example Plugin struct");
    }
}
