use super::*;

/// Build the Rust WASM app and all of its assets.
#[derive(Clone, Debug, Deserialize, Subcommand)]
#[clap(name = "plugin")]
pub enum Plugin {
    /// Return all dioxus-cli support tools.
    List {},
    /// Get default app install path.
    AppPath {},
    /// Install a new tool.
    Add {
        #[clap(long, default_value_t)]
        git: String,
        #[clap(long, default_value = "main")]
        branch: String,
    },
}

impl Plugin {
    pub async fn plugin(self) -> Result<()> {
        match self {
            Plugin::List {} => {
                for item in crate::plugin::PluginManager::plugin_list() {
                    println!("- {item}");
                }
            }
            Plugin::AppPath {} => {
                if let Some(v) = crate::plugin::PluginManager::init_plugin_dir().to_str() {
                    println!("{}", v);
                } else {
                    log::error!("Plugin path get failed.");
                }
            }
            Plugin::Add { git, branch } => {
                if !git.is_empty() {
                    if let Err(e) = crate::plugin::PluginManager::remote_install_plugin(git, branch) {
                        log::error!("Plugin install failed: {e}");
                    } else {
                        println!("🔰 Plugin install done.");
                    }
                } else {
                    println!("Please use `dioxus plugin add --git {{GIT_URL}}` to install plugin.\n");
                    log::warn!("We are working for plugin index system, but for now, you need use git url to install plugin.\n");
                    println!("Maybe this link can help you to find some useful plugins: https://github.com/search?q=dioxus-plugin&type=repositories")
                }
            }
        }
        Ok(())
    }
}
