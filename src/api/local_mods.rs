use std::fs;
use std::path::{Path, PathBuf};
use crate::api::settings::{ApiProvider, AppSettings, InstalledModEntry};

fn get_mods_dir(hytale_folder: &Path) -> PathBuf {
    hytale_folder.join("UserData").join("Mods")
}

pub fn extract_base_name(filename: &str) -> (String, String) {
    let name_without_extension = filename
        .strip_suffix(".jar")
        .unwrap_or_else(|| filename.strip_suffix(".zip").unwrap_or(filename));

    let split_index = name_without_extension
        .char_indices()
        .find(|(i, c)| {
            if *c == '-' {
                if let Some(next_char) = name_without_extension[i + 1..].chars().next() {
                    return next_char.is_numeric() || next_char == 'v';
                }
            }
            false
        })
        .map(|(i, _)| i);

    match split_index {
        Some(idx) => {
            let (name, version_with_dash) = name_without_extension.split_at(idx);
            let version = &version_with_dash[1..];
            (name.to_string(), version.to_string())
        }
        None => (name_without_extension.to_string(), "Unknown".to_string()),
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum InstallStatus {
    Installed,
    #[default]
    NotInstalled,
    Outdated,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ModInstallInfo {
    pub mod_name: String,
    pub local_version: Option<String>,
    pub local_file_name: Option<String>,
    pub install_status: InstallStatus,
}

impl ModInstallInfo {
    pub fn from_id(
        settings: &AppSettings,
        mod_id: &str,
        latest_file_id: &str
    ) -> Self {
        check_install_status(settings, mod_id, latest_file_id)
    }
}

pub fn check_install_status(
    settings: &AppSettings,
    mod_id: &str,
    latest_file_id: &str
) -> ModInstallInfo {
    let entry_opt = settings.installed_mods.values().find(|e| e.mod_id == mod_id);

    if let Some(entry) = entry_opt {
        let status = if entry.file_id == latest_file_id {
            InstallStatus::Installed
        } else {
            InstallStatus::Outdated
        };

        let file_name = settings.installed_mods.iter()
            .find(|(_, v)| v.mod_id == mod_id)
            .map(|(k, _)| k.clone());

        ModInstallInfo {
            mod_name: entry.mod_name.clone(),
            local_version: Some(entry.version_name.clone()),
            local_file_name: file_name,
            install_status: status,
        }
    } else {
        ModInstallInfo {
            mod_name: "".to_string(),
            local_version: None,
            local_file_name: None,
            install_status: InstallStatus::NotInstalled,
        }
    }
}

pub fn install_mod(
    folder: &Path,
    file_name: &str,
    data: &[u8],
    mod_id: String,
    mod_name: String,
    file_id: String,
    version_name: String,
    api_provider: ApiProvider,
    settings: &mut AppSettings,
) -> Result<(), String> {
    let mods_dir = get_mods_dir(folder);
    if !mods_dir.exists() {
        fs::create_dir_all(&mods_dir).map_err(|e| e.to_string())?;
    }

    if let Some((old_filename, _)) = settings.installed_mods.iter().find(|(_, v)| v.mod_id == mod_id) {
        let old_path = mods_dir.join(old_filename);
        if old_path.exists() {
            let _ = fs::remove_file(old_path);
        }

        let old_key = old_filename.clone();
        settings.remove_installed_mod(&old_key);
    }

    fs::write(mods_dir.join(file_name), data).map_err(|e| e.to_string())?;

    let entry = InstalledModEntry {
        mod_id,
        mod_name,
        file_id,
        version_name,
        api_provider,
    };
    settings.add_installed_mod(file_name.to_string(), entry);

    Ok(())
}

pub fn remove_mod(folder: &Path, file_name: &str, settings: &mut AppSettings) -> Result<(), String> {
    let mods_dir = get_mods_dir(folder);

    let path = mods_dir.join(file_name);
    if path.exists() {
        fs::remove_file(path).map_err(|e| e.to_string())?;
    }
    settings.remove_installed_mod(file_name);

    Ok(())
}