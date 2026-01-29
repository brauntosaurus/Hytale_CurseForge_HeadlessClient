use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::api::local_mods::{ModInstallInfo, check_install_status, InstallStatus};
use crate::api::settings::AppSettings;

#[derive(Clone, Copy)]
pub struct ModStore {
    pub status_cache: Signal<HashMap<String, ModInstallInfo>>,
    pub processing_ids: Signal<HashSet<String>>,
    pub refresh_trigger: Signal<u32>,
}

impl ModStore {
    pub fn new() -> Self {
        Self {
            status_cache: Signal::new(HashMap::new()),
            processing_ids: Signal::new(HashSet::new()),
            refresh_trigger: Signal::new(0),
        }
    }

    pub fn get_status(&self, mod_id: &str) -> InstallStatus {
        if let Some(info) = self.status_cache.read().get(mod_id) {
            return info.install_status.clone();
        }
        InstallStatus::NotInstalled
    }

    pub fn get_info(&mut self, mod_id: &str, latest_file_id: &str, settings: &AppSettings) -> ModInstallInfo {
        if let Some(info) = self.status_cache.read().get(mod_id) {
            return info.clone();
        }

        let info = check_install_status(settings, mod_id, latest_file_id);

        if info.install_status != InstallStatus::NotInstalled {
            self.status_cache.write().insert(mod_id.to_string(), info.clone());
        }

        info
    }

    pub fn set_info(&mut self, mod_id: &str, install_info: ModInstallInfo) {
        self.status_cache.write().insert(mod_id.to_string(), install_info);
    }

    pub fn remove_mod(&mut self, mod_id: &str) {
        self.status_cache.write().remove(mod_id);
    }

    pub fn is_processing(&self, mod_id: &str) -> bool {
        self.processing_ids.read().contains(mod_id)
    }

    pub fn set_processing(&mut self, mod_id: &str, is_loading: bool) {
        if is_loading {
            self.processing_ids.write().insert(mod_id.to_string());
        } else {
            self.processing_ids.write().remove(mod_id);
        }
    }

    pub fn trigger_refresh(&mut self) {
        self.refresh_trigger += 1;
    }
}