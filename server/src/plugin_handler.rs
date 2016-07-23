//use glob;
use libloading::os::unix::{Library, Symbol};
use std::collections::HashMap;

#[cfg(target_os="macos")]
const LIB_POSTFIX: &'static str = ".dylib";
#[cfg(target_os="linux")]
const LIB_POSTFIX: &'static str = ".so";

pub struct PluginHandler {
    plugins: HashMap<String, Library>,
    search_path: String
}

impl PluginHandler {
    pub fn new(search_path: String) -> PluginHandler {
        PluginHandler {
            plugins: HashMap::new(),
            search_path: search_path
        }
    }

    pub fn load(&mut self, name: String) -> bool {
        match Library::new(self.search_path.clone() + "/" + &name + LIB_POSTFIX) {
            Ok(plugin) => {
                self.plugins.insert(name, plugin);
                true
            },
            Err(err) => {
                println!("Error while loading plugin: {:?}", err);
                false
            }
        }
    }

    pub fn get_symbol<T>(&self, name: String, symbol_name: &str) -> Option<Symbol<T>> {
        match self.plugins.get(&name) {
            Some(plugin) => unsafe {
                match plugin.get(symbol_name.as_bytes()) {
                    Ok(symbol) => Some(symbol),
                    _ => None
                }
            },
            None => None
        }
    }

}
