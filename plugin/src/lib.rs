use libloading::{Library, Symbol};

// Defines the Plugin trait that all plugins must implement
pub trait Plugin {
    fn compile(&self, code: &str) -> Result<(), String>;
}

// Helper macro to export a plugin
#[macro_export]
macro_rules! export_plugin {
    ($plugin:expr) => {
        #[no_mangle]
        pub extern "C" fn new_plugin() -> *mut dyn Plugin {
            Box::into_raw(Box::new($plugin))
        }
    };
}

// Helper function to load a plugin from a shared library
pub fn load_plugin(path: &str) -> Result<Box<dyn Plugin>, String> {
    unsafe {
        let lib = Library::new(path).map_err(|e| e.to_string())?;
        let new_plugin: Symbol<fn() -> *mut dyn Plugin> =
            lib.get(b"new_plugin").map_err(|e| e.to_string())?;
        let raw_plugin = new_plugin();
        if raw_plugin.is_null() {
            Err("Failed to create plugin".into())
        } else {
            Ok(Box::from_raw(raw_plugin))
        }
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_plugin() {
        let plugin = load_plugin("../target/debug/libbuiltins.dylib");
        let code = r#"
          let x = 2;
          let y = 4;
          console.log(x + y);
        "#;
        match plugin {
            Ok(plugin) => {
                let result = plugin.compile(code);
                match result {
                    Ok(_) => assert!(true),
                    Err(msg) => {
                        dbg!(msg);
                        assert!(false)
                    }
                }
            }
            Err(msg) => {
                dbg!(msg);
                assert!(false)
            }
        }
    }
}
