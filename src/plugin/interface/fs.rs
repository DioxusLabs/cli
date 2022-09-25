use std::{
    fs::{create_dir, create_dir_all, remove_dir_all},
    path::PathBuf, io::{Read, Write},
};

use mlua::UserData;

use crate::tools::extract_zip;

pub struct PluginFileSystem;
impl UserData for PluginFileSystem {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("create_dir", |_, args: (String, bool)| {
            let path = args.0;
            let recursive = args.1;
            let path = PathBuf::from(path);
            if !path.exists() {
                let v = if recursive {
                    create_dir_all(path)
                } else {
                    create_dir(path)
                };
                return Ok(v.is_ok());
            }
            Ok(true)
        });
        methods.add_function("remove_dir", |_, path: String| {
            let path = PathBuf::from(path);
            let r = remove_dir_all(path);
            Ok(r.is_ok())
        });
        methods.add_function("file_get_content", |_, path: String| {
            let path = PathBuf::from(path);
            let mut file = std::fs::File::open(path)?;
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;
            Ok(buffer)
        });
        methods.add_function("file_set_content", |_, args: (String, String)| {
            let path = args.0;
            let content = args.1;
            let path = PathBuf::from(path);
            let mut file = std::fs::File::create(path)?;
            file.write_all(content.as_bytes())?;
            Ok(())
        });
        methods.add_function("unzip_file", |_, args: (String, String)| {
            let file = PathBuf::from(args.0);
            let target = PathBuf::from(args.1);
            let res = extract_zip(&file, &target);
            if let Err(e) = res {
                return Err(mlua::Error::RuntimeError(e.to_string()));
            }
            Ok(())
        });
    }
}