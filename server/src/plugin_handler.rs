use libloading::os::unix::{Library, Symbol};

#[cfg(target_os="macos")]
const LIB_POSTFIX: &'static str = ".dylib";
#[cfg(target_os="linux")]
const LIB_POSTFIX: &'static str = ".so";
const LIB_PREFIX: &'static str = "lib";

pub struct PluginHandler {
    search_path: String,
    plugins: Vec<(String, Library)>
}

impl PluginHandler {
    pub fn new(search_path: String) -> PluginHandler {
        PluginHandler {
            search_path: search_path,
            plugins: Vec::new()
        }
    }

    pub fn load(&mut self, username: String) -> bool {
        match Library::new(self.search_path.clone() + "/" + &LIB_PREFIX.to_string() + &username + LIB_POSTFIX) {
            Ok(plugin) => {
                self.plugins.push((username, plugin));
                true
            },
            Err(err) => {
                println!("Error while loading plugin: {:?}", err);
                false
            }
        }
    }

    pub fn get_symbol<T>(&self, name: String, symbol_name: &str) -> Option<Symbol<T>> {
        unsafe {
            let object = self.plugins.iter().find(|r| r.0 == name);
            match object {
                Some(plugin) => {
                    match plugin.1.get(symbol_name.as_bytes()) {
                        Ok(symbol) => Some(symbol),
                        _ => None
                    }
                },
                _ => None
            }
        }
    }

}
