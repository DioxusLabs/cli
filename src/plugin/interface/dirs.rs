use mlua::UserData;

use crate::crate_root;

pub struct PluginDirs;
impl UserData for PluginDirs {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("crate_dir", |_, ()| {
            let path = crate_root().unwrap();
            Ok(String::from(path.to_str().unwrap()))
        });
    }
}
