use std::fs::File;
use std::io::copy;
use std::path::PathBuf;

use mlua::{UserData, Result, Error, UserDataMethods};

use reqwest::Url;
use tokio::task;
use log::{error, info};

pub struct PluginNetwork;
impl UserData for PluginNetwork {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("download_file", |_, (url, path): (String, String)| {
            download_file(url, path)
        });
    }
}

fn download_file(url: String, path: String) -> Result<()> {
    let new_url = Url::parse(&url).map_err(|e| Error::external(e))?;
    let path = PathBuf::from(&path);

    task::block_in_place(|| {
        let mut response = match reqwest::blocking::get(new_url) {
            Ok(response) => response,
            Err(e) => {
                error!("Failed to download file from URL '{}': {}", url, e);
                return Err(Error::external(e));
            }
        };
        if !response.status().is_success() {
            let status = response.status();
            error!("Failed to download file from URL '{}': status {}", url, status);
            return Err(Error::external(format!("Failed to download file: status {}", status)));
        }
        let mut dest = match File::create(&path) {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to create file at path '{}': {}", path.display(), e);
                return Err(Error::external(e));
            }
        };
        if let Err(e) = copy(&mut response, &mut dest) {
            error!("Failed to write data to file at path '{}': {}", path.display(), e);
            return Err(Error::external(e));
        }
        info!("Downloaded file from URL '{}' to path '{}'", url, path.display());
        Ok(())
    })
}