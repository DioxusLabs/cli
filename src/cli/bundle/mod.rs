use std::{env::current_dir, str::FromStr};

use tauri_bundler::{BundleSettings, PackageSettings, SettingsBuilder};

use super::*;
use crate::{build_desktop, cfg::ConfigOptsBundle};

/// Build the Rust WASM app and all of its assets.
#[derive(Clone, Debug, Parser)]
#[clap(name = "bundle")]
pub struct Bundle {
    #[clap(long)]
    pub package: Option<Vec<String>>,
    #[clap(flatten)]
    pub build: ConfigOptsBundle,
}

#[derive(Clone, Debug)]
pub enum PackageType {
    MacOsBundle,
    IosBundle,
    WindowsMsi,
    Deb,
    Rpm,
    AppImage,
    Dmg,
    Updater,
}

impl FromStr for PackageType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "macos" => Ok(PackageType::MacOsBundle),
            "ios" => Ok(PackageType::IosBundle),
            "msi" => Ok(PackageType::WindowsMsi),
            "deb" => Ok(PackageType::Deb),
            "rpm" => Ok(PackageType::Rpm),
            "appimage" => Ok(PackageType::AppImage),
            "dmg" => Ok(PackageType::Dmg),
            _ => Err(format!("{} is not a valid package type", s)),
        }
    }
}

impl From<PackageType> for tauri_bundler::PackageType {
    fn from(val: PackageType) -> Self {
        match val {
            PackageType::MacOsBundle => tauri_bundler::PackageType::MacOsBundle,
            PackageType::IosBundle => tauri_bundler::PackageType::IosBundle,
            PackageType::WindowsMsi => tauri_bundler::PackageType::WindowsMsi,
            PackageType::Deb => tauri_bundler::PackageType::Deb,
            PackageType::Rpm => tauri_bundler::PackageType::Rpm,
            PackageType::AppImage => tauri_bundler::PackageType::AppImage,
            PackageType::Dmg => tauri_bundler::PackageType::Dmg,
            PackageType::Updater => tauri_bundler::PackageType::Updater,
        }
    }
}

impl Bundle {
    pub fn bundle(self) -> Result<()> {
        let mut crate_config = crate::CrateConfig::new()?;

        // change the release state.
        crate_config.with_release(self.build.release);
        crate_config.with_verbose(self.build.verbose);

        if self.build.example.is_some() {
            crate_config.as_example(self.build.example.unwrap());
        }

        if self.build.profile.is_some() {
            crate_config.set_profile(self.build.profile.unwrap());
        }

        // build the desktop app
        build_desktop(&crate_config, false)?;

        // copy the binary to the out dir
        let package = crate_config.manifest.package.unwrap();

        let mut name: PathBuf = match &crate_config.executable {
            crate::ExecutableType::Binary(name)
            | crate::ExecutableType::Lib(name)
            | crate::ExecutableType::Example(name) => name,
        }
        .into();
        if cfg!(windows) {
            name.set_extension("exe");
        }

        // bundle the app
        let binaries = vec![
            tauri_bundler::BundleBinary::new(name.display().to_string(), true)
                .set_src_path(Some(crate_config.crate_dir.display().to_string())),
        ];

        let settings = SettingsBuilder::new()
            .project_out_directory(crate_config.out_dir)
            .package_settings(PackageSettings {
                product_name: crate_config.dioxus_config.application.name.clone(),
                version: package.version,
                description: package.description.unwrap_or_default(),
                homepage: package.homepage,
                authors: Some(package.authors),
                default_run: Some(crate_config.dioxus_config.application.name.clone()),
            })
            .binaries(binaries)
            .bundle_settings(crate_config.dioxus_config.bundle.into())
            .package_types(
                self.package
                    .unwrap_or_default()
                    .into_iter()
                    .map(|p| p.parse::<PackageType>().unwrap().into())
                    .collect(),
            )
            .build();

        tauri_bundler::bundle::bundle_project(settings.unwrap()).unwrap();

        Ok(())
    }
}
