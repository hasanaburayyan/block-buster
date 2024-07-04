use plugin::{export_plugin, Plugin};

pub struct JsNode;

impl Plugin for JsNode {
    fn compile(&self, code: &str) -> Result<(), String> {
        println!("Compiling JS code: {}", code);
        Ok(())
    }
}

export_plugin!(JsNode);
