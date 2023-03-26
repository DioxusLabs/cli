use std::{
    io::{Read, Write},
    path::PathBuf,
    sync::Mutex, fs::create_dir,
};

use mlua::{Lua, Table};

use crate::{tools::clone_repo, CrateConfig, crate_root};

use self::{
    interface::{
        command::PluginCommander, dirs::PluginDirs, fs::PluginFileSystem, log::PluginLogger,
        network::PluginNetwork, os::PluginOS, path::PluginPath, PluginInfo,
    },
    types::PluginConfig, status::{get_plugin_status, set_plugin_status, PluginStatus},
};

pub mod interface;
pub mod status;
mod types;

lazy_static::lazy_static! {
    static ref LUA: Mutex<Lua> = Mutex::new(Lua::new());
}

pub struct PluginManager;

impl PluginManager {
    pub fn init(config: toml::Value) -> anyhow::Result<()> {
        let config = PluginConfig::from_toml_value(config);

        let lua = LUA.lock().expect("Lua runtime load failed");

        let manager = lua.create_table().expect("Lua runtime init failed");
        let name_index = lua.create_table().expect("Lua runtime init failed");

        let plugin_dir = Self::init_plugin_dir();

        let api = lua.create_table().unwrap();

        api.set("log", PluginLogger).expect("Plugin: `log` library init faield");
        api.set("command", PluginCommander).expect("Plugin: `command` library init faield");
        api.set("network", PluginNetwork).expect("Plugin: `network` library init faield");
        api.set("dirs", PluginDirs).expect("Plugin: `dirs` library init faield");
        api.set("fs", PluginFileSystem).expect("Plugin: `fs` library init faield");
        api.set("path", PluginPath).expect("Plugin: `path` library init faield");
        api.set("os", PluginOS).expect("Plugin: `os` library init faield");

        lua.globals().set("plugin_lib", api).expect("Plugin: library startup failed");
        lua.globals()
            .set("library_dir", plugin_dir.join("core").to_str().unwrap())
            .unwrap();
        lua.globals().set("config_info", config.clone())?;

        let mut index: u32 = 1;
        let dirs = std::fs::read_dir(&plugin_dir)?;

        let path_list = dirs
            .filter(|v| v.is_ok())
            .map(|v| (v.unwrap().path(), false))
            .collect::<Vec<(PathBuf, bool)>>();

        for entry in path_list {
            let plugin_dir = entry.0.to_path_buf();

            if plugin_dir.is_dir() {
                let init_file = plugin_dir.join("init.lua");
                if init_file.is_file() {
                    let mut file = std::fs::File::open(init_file).unwrap();
                    let mut buffer = String::new();
                    file.read_to_string(&mut buffer).unwrap();

                    let current_plugin_dir = plugin_dir.to_str().unwrap().to_string();
                    let from_loader = entry.1;

                    lua.globals()
                        .set("_temp_plugin_dir", current_plugin_dir.clone())?;
                    lua.globals().set("_temp_from_loader", from_loader)?;

                    let info = lua.load(&buffer).eval::<PluginInfo>();
                    match info {
                        Ok(mut info) => {
                            if name_index.contains_key(info.name.clone()).unwrap_or(false)
                                && !from_loader
                            {
                                // found same name plugin, intercept load
                                log::warn!(
                                    "Plugin `{}` has been intercepted. [mulit-load]",
                                    info.name
                                );
                                continue;
                            }
                            info.inner.plugin_dir = current_plugin_dir;
                            info.inner.from_loader = from_loader;

                            // call `on_init` if file "dcp.json" not exists
                            let plugin_status = get_plugin_status(&info.name);
                            if plugin_status.is_none() {
                                if let Some(func) = info.clone().on_init {
                                    let result = func.call::<_, bool>(());
                                    match result {
                                        Ok(true) => {
                                            set_plugin_status(&info.name, PluginStatus {
                                                version: info.version.clone(),
                                                startup_timestamp: chrono::Local::now().timestamp(),
                                            });

                                            // insert plugin-info into plugin-manager
                                            if let Ok(index) =
                                                name_index.get::<_, u32>(info.name.clone())
                                            {
                                                let _ = manager.set(index, info.clone());
                                            } else {
                                                let _ = manager.set(index, info.clone());
                                                index += 1;
                                                let _ = name_index.set(info.name, index);
                                            }
                                        }
                                        Ok(false) => {
                                            log::warn!(
                                                "Plugin rejected init, read plugin docs to get more details"
                                            );
                                        }
                                        Err(e) => {
                                            log::warn!("Plugin init failed: {e}");
                                        }
                                    }
                                }
                            } else {
                                if let Ok(index) = name_index.get::<_, u32>(info.name.clone()) {
                                    let _ = manager.set(index, info.clone());
                                } else {
                                    let _ = manager.set(index, info.clone());
                                    index += 1;
                                    let _ = name_index.set(info.name, index);
                                }
                            }
                        }
                        Err(_e) => {
                            let dir_name = plugin_dir.file_name().unwrap().to_str().unwrap();
                            log::error!("Plugin '{dir_name}' load failed.");
                            log::error!("Error Detail: {_e}")
                        }
                    }
                }
            }
        }

        lua.globals().set("manager", manager).unwrap();

        return Ok(());
    }

    pub fn on_build_start(crate_config: &CrateConfig, platform: &str) -> anyhow::Result<()> {
        let lua = LUA.lock().unwrap();

        if !lua.globals().contains_key("manager")? {
            return Ok(());
        }
        let manager = lua.globals().get::<_, Table>("manager")?;

        let args = lua.create_table()?;
        args.set("name", crate_config.dioxus_config.application.name.clone())?;
        args.set("platform", platform)?;
        args.set("out_dir", crate_config.out_dir.to_str().unwrap())?;
        args.set("asset_dir", crate_config.asset_dir.to_str().unwrap())?;

        for i in 1..(manager.len()? as i32 + 1) {
            let info = manager.get::<i32, PluginInfo>(i)?;
            if let Some(func) = info.build.on_start {
                func.call::<Table, ()>(args.clone())?;
            }
        }

        Ok(())
    }

    pub fn on_build_finish(crate_config: &CrateConfig, platform: &str) -> anyhow::Result<()> {
        let lua = LUA.lock().unwrap();

        if !lua.globals().contains_key("manager")? {
            return Ok(());
        }
        let manager = lua.globals().get::<_, Table>("manager")?;

        let args = lua.create_table()?;
        args.set("name", crate_config.dioxus_config.application.name.clone())?;
        args.set("platform", platform)?;
        args.set("out_dir", crate_config.out_dir.to_str().unwrap())?;
        args.set("asset_dir", crate_config.asset_dir.to_str().unwrap())?;

        for i in 1..(manager.len()? as i32 + 1) {
            let info = manager.get::<i32, PluginInfo>(i)?;
            if let Some(func) = info.build.on_finish {
                func.call::<Table, ()>(args.clone())?;
            }
        }

        Ok(())
    }

    pub fn on_serve_start(crate_config: &CrateConfig) -> anyhow::Result<()> {
        let lua = LUA.lock().expect("Lua runtime load failed.");

        if !lua.globals().contains_key("manager")? {
            return Ok(());
        }
        let manager = lua.globals().get::<_, Table>("manager")?;

        let args = lua.create_table()?;
        args.set("name", crate_config.dioxus_config.application.name.clone())?;

        for i in 1..(manager.len()? as i32 + 1) {
            let info = manager.get::<i32, PluginInfo>(i)?;
            if let Some(func) = info.serve.on_start {
                func.call::<Table, ()>(args.clone())?;
            }
        }

        Ok(())
    }

    pub fn on_serve_rebuild(timestamp: i64, files: Vec<PathBuf>) -> anyhow::Result<()> {
        let lua = LUA.lock().expect("Lua runtime load failed.");

        let manager = lua.globals().get::<_, Table>("manager")?;

        let args = lua.create_table()?;
        args.set("timestamp", timestamp)?;
        let files: Vec<String> = files
            .iter()
            .map(|v| v.to_str().unwrap().to_string())
            .collect();
        args.set("changed_files", files)?;

        for i in 1..(manager.len()? as i32 + 1) {
            let info = manager.get::<i32, PluginInfo>(i)?;
            if let Some(func) = info.serve.on_rebuild {
                func.call::<Table, ()>(args.clone())?;
            }
        }

        Ok(())
    }

    pub fn on_serve_shutdown(crate_config: &CrateConfig) -> anyhow::Result<()> {
        let lua = LUA.lock().expect("Lua runtime load failed.");

        if !lua.globals().contains_key("manager")? {
            return Ok(());
        }
        let manager = lua.globals().get::<_, Table>("manager")?;

        let args = lua.create_table()?;
        args.set("name", crate_config.dioxus_config.application.name.clone())?;

        for i in 1..(manager.len()? as i32 + 1) {
            let info = manager.get::<i32, PluginInfo>(i)?;
            if let Some(func) = info.serve.on_shutdown {
                func.call::<Table, ()>(args.clone())?;
            }
        }

        Ok(())
    }

    pub fn init_plugin_dir() -> PathBuf {
        let plugin_path = crate_root().unwrap().join(".dioxus").join("plugins");
        if !plugin_path.is_dir() {
            create_dir(&plugin_path).expect("Create plugin directory failed.");
            let mut plugin_lock_file = std::fs::File::create(plugin_path.join("Plugin.lock")).expect("Plugin file init failed.");
            let content = "{}".as_bytes();
            plugin_lock_file.write_all(content).expect("Plugin file init failed.");
        }
        let core_path = plugin_path.join("core");
        if !core_path.is_dir() {
            log::info!("📖 Start to init plugin library ...");
            let url = "https://github.com/DioxusLabs/cli-plugin-library";
            clone_repo(&core_path, url, "v2").expect("Init Plugin Library faield.");
            log::info!("🔰 Plugin library dowonload done.");
        }
        plugin_path
    }

    pub fn plugin_list() -> Vec<String> {
        let mut res = vec![];

        if let Ok(lua) = LUA.lock() {
            let list = lua
                .load(mlua::chunk!(
                    local list = {}
                    for key, value in ipairs(manager) do
                        table.insert(list, {name = value.name, version = value.version})
                    end
                    return list
                ))
                .eval::<Vec<Table>>()
                .unwrap_or_default();
            for i in list {
                let name = i.get::<_, String>("name").unwrap();
                let version = i.get::<_, String>("version").unwrap();
                let text = format!("{name} [{version}]");
                res.push(text);
            }
        }

        res
    }

    pub fn remote_install_plugin(url: String, branch: String) -> anyhow::Result<()> {
        let plugin_dir = Self::init_plugin_dir();
        let binding = url.split("/").collect::<Vec<&str>>();
        let repo_name = binding.last().unwrap();

        let target_path = plugin_dir.join(repo_name);

        if target_path.is_dir() {
            return Err(anyhow::anyhow!("Plugin directory exist."));
        }

        clone_repo(&target_path, &url, &branch)?;
        Ok(())
    }

}
