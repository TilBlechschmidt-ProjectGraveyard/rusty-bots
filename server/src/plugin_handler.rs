use libloading::os::unix::{Library, Symbol};
use std::collections::HashMap;

#[cfg(target_os="macos")]
const LIB_POSTFIX: &'static str = ".dylib";
#[cfg(target_os="linux")]
const LIB_POSTFIX: &'static str = ".so";

pub struct Plugin {
    plugin: Library
}

impl Plugin {
    pub fn get_welcome(&self) -> Option<Symbol<unsafe fn() -> String>> {
        unsafe {
            match self.plugin.get("welcome".as_bytes()) {
                Ok(symbol) => Some(symbol),
                _ => None
            }
        }
    }
}


pub struct PluginHandler {
    search_path: String
}

impl PluginHandler {
    pub fn new(search_path: String) -> PluginHandler {
        PluginHandler {
            search_path: search_path
        }
    }

    pub fn load(&mut self, name: String) -> Option<Plugin> {
        match Library::new(self.search_path.clone() + "/" + &name + LIB_POSTFIX) {
            Ok(plugin) => {
                Some(
                    Plugin {
                        plugin: plugin
                    }
                )
            },
            Err(err) => {
                println!("Error while loading plugin: {:?}", err);
                None
            }
        }
    }

    // pub fn get_symbol<T>(&self, name: String, symbol_name: &str) -> Option<Symbol<T>> {
    //     // let index = self.users.iter().position(|&r| r == name.clone()).unwrap();
    //     match self.users.iter().position(|r| r == &name) {
    //         Some(index) => unsafe {
    //             let plugin = self.plugins;
    //             match plugin.get(symbol_name.as_bytes()) {
    //                 Ok(symbol) => Some(symbol),
    //                 _ => None
    //             }
    //         },
    //         None => None
    //     }
    // }

}
