use crate::api::ui_mod::get_mod_details_unified;
use crate::api::local_mods::extract_base_name;
use crate::api::settings::AppSettings;
use crate::api::ui_mod::{UiMod, UiModVersion};
use crate::components::mod_card::ModCard;
use crate::state::mod_store::ModStore;
use dioxus::prelude::*;
use std::fs;
use std::path::Path;

#[component]
pub fn InstalledPage(on_open_info: EventHandler<UiMod>) -> Element {
    let app_settings = use_context::<Signal<AppSettings>>();

    let mut refresh_trigger = use_signal(|| 0);
    let mut query = use_signal(|| String::new());
    let mut display_list = use_signal(|| Vec::<UiMod>::new());
    let mut is_scanning = use_signal(|| false);

    use_resource(move || async move {
        let _ = refresh_trigger();
        is_scanning.set(true);

        let (folder_opt, installed_map) = {
            let s = app_settings.peek();
            (s.get_game_folder(), s.installed_mods.clone())
        };

        let mut new_list: Vec<UiMod> = Vec::new();

        if let Some(folder) = folder_opt {
            let mods_path = Path::new(&folder).join("UserData").join("Mods");

            if !mods_path.exists() {
                let _ = fs::create_dir_all(&mods_path);
            }

            if let Ok(entries) = fs::read_dir(&mods_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() && path.extension().map_or(false, |ext| ext == "jar" || ext == "zip") {
                        let filename = entry.file_name().to_string_lossy().into_owned();

                        if let Some(known_mod) = installed_map.get(&filename) {
                            if let Some(ui_mod) = get_mod_details_unified(&known_mod.api_provider, &known_mod.mod_id).await {
                                new_list.push(ui_mod);
                                continue;
                            }
                        }

                        let (base_name, local_version) = extract_base_name(&filename);

                        let (id, display_name) = if let Some(known) = installed_map.get(&filename) {
                            (known.mod_id.clone(), known.mod_name.clone())
                        } else {
                            ("0".to_string(), base_name.replace("-", " "))
                        };

                        new_list.push(UiMod {
                            id,
                            name: display_name,
                            summary: format!("Local file: {}", filename),
                            authors: "Local Install".to_string(),
                            download_count: 0,
                            icon: String::new(),
                            categories: vec![],
                            version: UiModVersion {
                                file_id: "0".to_string(),
                                file_name: filename.clone(),
                                display_name: local_version,
                                download_url: None,
                                release_type: 1,
                                upload_date: "Local".to_string(),
                                game_versions: vec![],
                            },
                            gallery_urls: vec![],
                            website_url: String::new(),
                            banner: String::new(),
                        });
                    }
                }
            }
        }

        new_list.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        display_list.set(new_list);
        is_scanning.set(false);
    });

    let filtered_mods = use_memo(move || {
        let mods = display_list.read();
        let q = query().to_lowercase();
        if q.is_empty() {
            mods.clone()
        } else {
            mods.iter()
                .filter(|m| m.name.to_lowercase().contains(&q) || m.version.file_name.to_lowercase().contains(&q))
                .cloned()
                .collect()
        }
    });

    rsx! {
        div { style: "display: flex; flex-direction: column; height: 100%;",
            h2 { style: "color: var(--text-primary); margin-top: 0;", "Installed Mods" }
            div { style: "display: flex; gap: 10px; margin-bottom: 20px;",
                input {
                    style: "flex: 1;",
                    placeholder: "Filter installed mods...",
                    value: "{query}",
                    oninput: move |e| query.set(e.value()),
                }
                button {
                    class: "btn btn-secondary",
                    onclick: move |_| refresh_trigger += 1,
                    "Refresh"
                }
            }

            div { style: "flex: 1; overflow-y: auto; padding-right: 5px; margin-bottom: 10px;",
                if is_scanning() {
                     div { style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 50%; color: var(--text-secondary); gap: 10px;",
                         div { class: "spinner" }
                         span { "Scanning Mods..." }
                     }
                } else if display_list.read().is_empty() {
                    div { style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 50%; color: var(--text-secondary); gap: 10px;",
                        if app_settings.read().get_game_folder().is_none() {
                             span { style: "color: var(--danger);", "⚠ No Game Folder Set" }
                        } else {
                            span { "No mods found in folder." }
                        }
                    }
                } else {
                    div { style: "display: flex; flex-direction: column; gap: 10px;",
                        for m in filtered_mods().into_iter() {
                            ModCard {
                                key: "{m.version.file_name}",
                                mod_data: m,
                                onclick: on_open_info
                            }
                        }
                    }
                }
            }
            div { style: "padding: 10px; background-color: var(--bg-tertiary); border-radius: 8px; font-size: 12px; color: var(--text-secondary);",
                if let Some(folder) = app_settings.read().get_game_folder() { "Location: {folder.display()}" } else { "No folder selected" }
            }
        }
    }
}