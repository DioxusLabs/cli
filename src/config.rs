use crate::{error::Result, plugin::types::PluginConfig};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DioxusConfig {
    pub application: ApplicationConfig,

    pub web: WebConfig,

    #[serde(default = "default_plugin")]
    pub plugin: toml::Value,
}

fn default_plugin() -> toml::Value {
    toml::Value::Boolean(true)
}

// this trait implement for DioxusConfig transfer to Lua plugin
impl<'lua> mlua::ToLua<'lua> for DioxusConfig {
    fn to_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        let data = lua.create_table()?;

        let plugin_config = PluginConfig::from_toml_value(self.plugin);
        data.set("plugin", plugin_config)?;

        let application_table = {
            let tab = lua.create_table()?;

            tab.set("name", self.application.name)?;
            tab.set("default_patform", self.application.default_platform)?;
            if let Some(out_dir) = self.application.out_dir {
                tab.set("out_dir", out_dir.to_str().unwrap().to_string())?;
            }
            if let Some(asset_dir) = self.application.asset_dir {
                tab.set("asset_dir", asset_dir.to_str().unwrap().to_string())?;
            }
            tab.set("sub_package", self.application.sub_package)?;
            tab
        };
        data.set("application", application_table)?;

        let web_table = {
            let tab = lua.create_table()?;

            let app = lua.create_table()?;
            app.set("title", self.web.app.title)?;
            app.set("base_path", self.web.app.base_path)?;
            tab.set("app", app)?;

            let watcher = lua.create_table()?;
            watcher.set("reload_html", self.web.watcher.reload_html)?;
            watcher.set("index_on_404", self.web.watcher.index_on_404)?;
            watcher.set(
                "watch_path",
                self.web.watcher.watch_path.map(|l| {
                    l.iter()
                        .map(|p| p.to_str().unwrap().to_string())
                        .collect::<Vec<String>>()
                }),
            )?;
            tab.set("watcher", watcher)?;

            let resource = lua.create_table()?;
            resource.set(
                "style",
                self.web.resource.style.map(|i| {
                    i.iter()
                        .map(|p| p.to_str().unwrap().to_string())
                        .collect::<Vec<String>>()
                }),
            )?;
            resource.set(
                "script",
                self.web.resource.script.map(|i| {
                    i.iter()
                        .map(|p| p.to_str().unwrap().to_string())
                        .collect::<Vec<String>>()
                }),
            )?;

            let dev_resource = lua.create_table()?;
            dev_resource.set(
                "style",
                self.web.resource.dev.style.map(|i| {
                    i.iter()
                        .map(|p| p.to_str().unwrap().to_string())
                        .collect::<Vec<String>>()
                }),
            )?;
            dev_resource.set(
                "script",
                self.web.resource.dev.script.map(|i| {
                    i.iter()
                        .map(|p| p.to_str().unwrap().to_string())
                        .collect::<Vec<String>>()
                }),
            )?;
            tab.set("dev", dev_resource)?;

            let proxy = self.web.proxy.map(|v| {
                let it = v.iter().map(|v| {
                    let temp = lua.create_table().unwrap();
                    temp.set("backend", v.clone().backend).unwrap();
                    temp
                });
                let mut result = Vec::new();
                for i in it {
                    result.push(i);
                }
                result
            });
            tab.set("proxy", proxy)?;

            tab.set("resource", resource)?;

            tab
        };
        data.set("web", web_table)?;

        Ok(mlua::Value::Table(data))
    }
}

impl DioxusConfig {
    pub fn load() -> crate::error::Result<Option<DioxusConfig>> {
        let Ok(crate_dir) = crate::cargo::crate_root() else { return Ok(None); };

        // we support either `Dioxus.toml` or `Cargo.toml`
        let Some(dioxus_conf_file) = acquire_dioxus_toml(&crate_dir) else {
            return Ok(None);
        };

        // init .dioxus folder for project
        let dioxus_dir = crate_dir.join(".dioxus");
        if !dioxus_dir.is_dir() {
            std::fs::create_dir(&dioxus_dir)?;
        }

        toml::from_str::<DioxusConfig>(&std::fs::read_to_string(dioxus_conf_file)?)
            .map_err(|_| crate::Error::Unique("Dioxus.toml parse failed".into()))
            .map(Some)
    }
}

fn acquire_dioxus_toml(dir: &PathBuf) -> Option<PathBuf> {
    // prefer uppercase
    if dir.join("Dioxus.toml").is_file() {
        return Some(dir.join("Dioxus.toml"));
    }

    // lowercase is fine too
    if dir.join("dioxus.toml").is_file() {
        return Some(dir.join("Dioxus.toml"));
    }

    None
}

impl Default for DioxusConfig {
    fn default() -> Self {
        Self {
            application: ApplicationConfig {
                name: "dioxus".into(),
                default_platform: "web".to_string(),
                out_dir: Some(PathBuf::from("dist")),
                asset_dir: Some(PathBuf::from("public")),

                tools: None,

                sub_package: None,
            },
            web: WebConfig {
                app: WebAppConfig {
                    title: Some("dioxus | ⛺".into()),
                    base_path: None,
                },
                proxy: Some(vec![]),
                watcher: WebWatcherConfig {
                    watch_path: Some(vec![PathBuf::from("src")]),
                    reload_html: Some(false),
                    index_on_404: Some(true),
                },
                resource: WebResourceConfig {
                    dev: WebDevResourceConfig {
                        style: Some(vec![]),
                        script: Some(vec![]),
                    },
                    style: Some(vec![]),
                    script: Some(vec![]),
                },
            },
            plugin: toml::Value::Table(toml::map::Map::new()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationConfig {
    pub name: String,
    pub default_platform: String,
    pub out_dir: Option<PathBuf>,
    pub asset_dir: Option<PathBuf>,

    pub tools: Option<HashMap<String, toml::Value>>,

    pub sub_package: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebConfig {
    pub app: WebAppConfig,
    pub proxy: Option<Vec<WebProxyConfig>>,
    pub watcher: WebWatcherConfig,
    pub resource: WebResourceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAppConfig {
    pub title: Option<String>,
    pub base_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebProxyConfig {
    pub backend: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebWatcherConfig {
    pub watch_path: Option<Vec<PathBuf>>,
    pub reload_html: Option<bool>,
    pub index_on_404: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebResourceConfig {
    pub dev: WebDevResourceConfig,
    pub style: Option<Vec<PathBuf>>,
    pub script: Option<Vec<PathBuf>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebDevResourceConfig {
    pub style: Option<Vec<PathBuf>>,
    pub script: Option<Vec<PathBuf>>,
}

#[derive(Debug, Clone)]
pub struct CrateConfig {
    pub out_dir: PathBuf,
    pub crate_dir: PathBuf,
    pub workspace_dir: PathBuf,
    pub target_dir: PathBuf,
    pub asset_dir: PathBuf,
    pub manifest: cargo_toml::Manifest<cargo_toml::Value>,
    pub executable: ExecutableType,
    pub dioxus_config: DioxusConfig,
    pub release: bool,
    pub hot_reload: bool,
    pub cross_origin_policy: bool,
    pub verbose: bool,
    pub custom_profile: Option<String>,
    pub features: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub enum ExecutableType {
    Binary(String),
    Lib(String),
    Example(String),
}

impl CrateConfig {
    pub fn new() -> Result<Self> {
        let dioxus_config = DioxusConfig::load()?.unwrap_or_default();

        let crate_dir = if let Some(package) = &dioxus_config.application.sub_package {
            crate::cargo::crate_root()?.join(package)
        } else {
            crate::cargo::crate_root()?
        };
        let meta = crate::cargo::Metadata::get()?;
        let workspace_dir = meta.workspace_root;
        let target_dir = meta.target_directory;

        let out_dir = match dioxus_config.application.out_dir {
            Some(ref v) => crate_dir.join(v),
            None => crate_dir.join("dist"),
        };

        let cargo_def = &crate_dir.join("Cargo.toml");

        let asset_dir = match dioxus_config.application.asset_dir {
            Some(ref v) => crate_dir.join(v),
            None => crate_dir.join("public"),
        };

        let manifest = cargo_toml::Manifest::from_path(&cargo_def).unwrap();

        // We just assume they're using a 'main.rs'
        // Anyway, we've already parsed the manifest, so it should be easy to change the type
        let output_filename = manifest
            .bin
            .first()
            .or(manifest.lib.as_ref())
            .and_then(|product| product.name.clone())
            .or_else(|| manifest.package.as_ref().map(|pkg| pkg.name.clone()))
            .expect("No lib found from cargo metadata");
        let executable = ExecutableType::Binary(output_filename);

        let release = false;
        let hot_reload = false;
        let verbose = false;
        let custom_profile = None;
        let features = None;

        Ok(Self {
            out_dir,
            crate_dir,
            workspace_dir,
            target_dir,
            asset_dir,
            manifest,
            executable,
            release,
            dioxus_config,
            hot_reload,
            cross_origin_policy: false,
            custom_profile,
            features,
            verbose,
        })
    }

    pub fn as_example(&mut self, example_name: String) -> &mut Self {
        self.executable = ExecutableType::Example(example_name);
        self
    }

    pub fn with_release(&mut self, release: bool) -> &mut Self {
        self.release = release;
        self
    }

    pub fn with_hot_reload(&mut self, hot_reload: bool) -> &mut Self {
        self.hot_reload = hot_reload;
        self
    }

    pub fn with_cross_origin_policy(&mut self, cross_origin_policy: bool) -> &mut Self {
        self.cross_origin_policy = cross_origin_policy;
        self
    }

    pub fn with_verbose(&mut self, verbose: bool) -> &mut Self {
        self.verbose = verbose;
        self
    }

    pub fn set_profile(&mut self, profile: String) -> &mut Self {
        self.custom_profile = Some(profile);
        self
    }

    pub fn set_features(&mut self, features: Vec<String>) -> &mut Self {
        self.features = Some(features);
        self
    }
}
