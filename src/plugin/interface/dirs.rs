use mlua::UserData;

// use crate::crate_root;

pub struct PluginDirs;
impl UserData for PluginDirs {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(_methods: &mut M) {
        // todo!()
    }
}
