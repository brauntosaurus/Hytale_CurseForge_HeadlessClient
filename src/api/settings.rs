use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::io;

use crate::api::curse_forge_api::set_global_api_key as set_curseforge_key;
use crate::api::mod_tale_api::set_global_api_key as set_modtale_key;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ApiProvider {
    Modtale,
    CurseForge,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AppTheme {
    Dark,
    Light,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstalledModEntry {
    pub mod_id: String,
    pub mod_name: String,
    pub file_id: String,
    pub version_name: String,
    pub api_provider: ApiProvider,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub api_key: Option<String>,
    pub game_folder: Option<PathBuf>,
    pub theme: AppTheme,
    pub api_provider: ApiProvider,
    pub installed_mods: HashMap<String, InstalledModEntry>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            api_key: None,
            game_folder: None,
            theme: AppTheme::Dark,
            api_provider: ApiProvider::CurseForge,
            installed_mods: HashMap::new(),
        }
    }
}

impl AppSettings {
    pub fn load() -> Self {
        let path = Self::get_config_path();
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    match serde_json::from_str::<AppSettings>(&content) {
                        Ok(settings) => {
                            println!("Loaded settings from: {:?}", path);

                            let key = settings.api_key.as_deref().unwrap_or_default();
                            match settings.api_provider {
                                ApiProvider::CurseForge => set_curseforge_key(key),
                                ApiProvider::Modtale => set_modtale_key(key)
                            }
                            return settings;
                        },
                        Err(e) => eprintln!("Failed to parse settings.json: {}. Using defaults.", e),
                    }
                }
                Err(e) => eprintln!("Failed to read settings file: {}. Using defaults.", e),
            }
        } else {
            let default_settings = AppSettings::default();
            if let Err(e) = default_settings.save() {
                eprintln!("Failed to create initial settings file: {}", e);
            }
            return default_settings;
        }

        AppSettings::default()
    }

    pub fn save(&self) -> io::Result<()> {
        let path = Self::get_config_path();

        let key = self.api_key.as_deref().unwrap_or_default();
        match self.api_provider {
            ApiProvider::CurseForge => set_curseforge_key(key),
            ApiProvider::Modtale => set_modtale_key(key)
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;

        Ok(())
    }

    fn get_config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| {
            PathBuf::from(".")
        });

        path.push("hytale-mod-manager");
        path.push("settings.json");
        path
    }

    pub fn change_api(&mut self, api_provider: ApiProvider, api_key: String) {

        match api_provider {
            ApiProvider::CurseForge => set_curseforge_key(&api_key),
            ApiProvider::Modtale => set_modtale_key(&api_key)
        }

        self.api_provider = api_provider;
        self.api_key = Some(api_key);
    }

    pub fn get_api_key(&self) -> Option<String> {
        self.api_key.clone()
    }
    pub fn get_api_provider(&self) -> ApiProvider {
        self.api_provider.clone()
    }

    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = Some(api_key);
    }
    pub fn set_api_provider(&mut self, api_provider: ApiProvider) {
        self.api_provider = api_provider;
    }

    pub fn get_game_folder(&self) -> Option<PathBuf> {
        self.game_folder.clone()
    }
    pub fn set_game_folder(&mut self, game_folder: Option<PathBuf>) {
        self.game_folder = game_folder;
    }

    pub fn get_theme(&self) -> AppTheme {
        self.theme.clone()
    }
    pub fn set_theme(&mut self, theme: AppTheme) {
        self.theme = theme;
    }
    pub fn switch_theme(&mut self) {
        match self.theme {
            AppTheme::Light => { self.theme = AppTheme::Dark  }
            AppTheme::Dark => { self.theme = AppTheme::Light }
        }
    }

    pub fn add_installed_mod(&mut self, filename: String, entry: InstalledModEntry) {
        self.installed_mods.insert(filename, entry);
        let _ = self.save();
    }

    pub fn remove_installed_mod(&mut self, filename: &str) {
        self.installed_mods.remove(filename);
        let _ = self.save();
    }

    pub fn get_installed_mod(&self, filename: &str) -> Option<&InstalledModEntry> {
        self.installed_mods.get(filename)
    }

    pub fn prune_manifest(&mut self) {
        if let Some(folder) = &self.game_folder {
            let mods_path = folder.join("UserData").join("Mods");

            let keys_to_remove: Vec<String> = self.installed_mods
                .keys()
                .filter(|filename| !mods_path.join(filename).exists())
                .cloned()
                .collect();

            if !keys_to_remove.is_empty() {
                for key in keys_to_remove {
                    self.installed_mods.remove(&key);
                }
                let _ = self.save();
            }
        }
    }
}