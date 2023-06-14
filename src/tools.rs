use std::{io::ErrorKind, path::Path, process::Command};

pub fn clone_repo(dir: &Path, url: &str, branch: &str) -> anyhow::Result<()> {
    let target_dir = dir.parent().unwrap();
    let dir_name = dir.file_name().unwrap();

    let mut cmd = Command::new("git");
    let cmd = cmd.current_dir(target_dir);
    let res = cmd
        .arg("clone")
        .arg("--branch")
        .arg(branch)
        .arg(url)
        .arg(dir_name)
        .output();
    if let Err(err) = res {
        if ErrorKind::NotFound == err.kind() {
            log::warn!("Git program not found. Hint: Install git or check $PATH.");
            return Err(err.into());
        }
    }
    Ok(())
}

pub fn extract_zip(file: &Path, target: &Path) -> anyhow::Result<()> {
    let zip_file = std::fs::File::open(&file)?;
    let mut zip = zip::ZipArchive::new(zip_file)?;

    if !target.exists() {
        let _ = std::fs::create_dir_all(target)?;
    }

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        if file.is_dir() {
            // dir
            let target = target.join(Path::new(&file.name().replace('\\', "")));
            let _ = std::fs::create_dir_all(target)?;
        } else {
            // file
            let file_path = target.join(Path::new(file.name()));
            let mut target_file = if !file_path.exists() {
                std::fs::File::create(file_path)?
            } else {
                std::fs::File::open(file_path)?
            };
            let _num = std::io::copy(&mut file, &mut target_file)?;
        }
    }

    Ok(())
}
