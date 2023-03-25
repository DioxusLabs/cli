use std::env::current_dir;

use mlua::UserData;

pub struct PluginDirs;
impl UserData for PluginDirs {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("plugins_dir", |_, ()| {
            let path = current_dir().unwrap().join(".dioxus").join("plugins");
            Ok(path.to_str().unwrap().to_string())
        });
    }
}
