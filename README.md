# Block Buster

The easiest way to test your code blocks in markdown files!

## Draft Sketches
Example of a plugin

```rust
use plugin_interface::Plugin;
use std::process::Command;

pub struct JsPlugin;

impl Plugin for JsPlugin {
    fn compile(&self, code: &str) -> Result<(), String> {
        let output = Command::new("node")
            .arg("--check")
            .arg("-e")
            .arg(code)
            .output()
            .expect("failed to execute process");

        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    fn execute(&self, _code: &str) -> Result<(), String> {
        Err("Execute method not implemented".into())
    }
}

export_plugin!(JsPlugin);
```