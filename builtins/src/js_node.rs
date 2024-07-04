extern crate plugin;
use plugin::{export_plugin, Plugin};
use std::process::Command;

pub struct JsNode;

impl Plugin for JsNode {
    fn compile(&self, code: &str) -> Result<(), String> {
        println!("Compiling JS code: {}", code);
        let output = Command::new("node")
            .arg("--check")
            .arg("-e")
            .arg(code)
            .output()
            .expect("Failed to execute command");
          if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
}

export_plugin!(JsNode);
