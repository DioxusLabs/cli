use std::fs::File;
use std::io::copy;
use std::path::PathBuf;

use mlua::{Error, UserData, UserDataMethods};

use log::error;
use reqwest::Url;
use tokio::task;

pub struct PluginNetwork;
impl UserData for PluginNetwork {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("download_file", |_, (url, path): (String, String)| {
            let new_url = Url::parse(&url).map_err(Error::external)?;
            let path = PathBuf::from(&path);

            task::block_in_place(|| {
                let mut response = match reqwest::blocking::get(new_url) {
                    Ok(response) => response,
                    Err(e) => {
                        error!("Failed to download file from URL '{}': {}", url, e);
                        return Ok(false);
                    }
                };
                if !response.status().is_success() {
                    let status = response.status();
                    error!(
                        "Failed to download file from URL '{}': status {}",
                        url, status
                    );
                    return Ok(false);
                }
                let mut dest = match File::create(&path) {
                    Ok(file) => file,
                    Err(e) => {
                        error!("Failed to create file at path '{}': {}", path.display(), e);
                        return Ok(false);
                    }
                };
                if let Err(e) = copy(&mut response, &mut dest) {
                    error!(
                        "Failed to write data to file at path '{}': {}",
                        path.display(),
                        e
                    );
                    return Ok(false);
                }
                Ok(true)
            })
        });
    }
}
