use dioxus::prelude::*;
use std::path::{Path, PathBuf};
use crate::api::settings::{AppSettings, AppTheme};
use crate::components::drop_down::{ThemeDropdown};
use crate::SidebarTab;

fn truncate_path(path: &Path, max_chars: usize) -> String {
    let s = path.to_string_lossy();
    if s.chars().count() <= max_chars {
        return s.to_string();
    }
    let chars: Vec<char> = s.chars().collect();
    let keep_count = max_chars.saturating_sub(3);
    let start_index = chars.len().saturating_sub(keep_count);
    let end_part: String = chars[start_index..].iter().collect();
    format!("...{}", end_part)
}

#[component]
pub fn Sidebar(
    active_tab: Signal<SidebarTab>,
    on_open_api: EventHandler<()>,
) -> Element {
    let mut settings_store = use_context::<Signal<AppSettings>>();

    let game_folder = settings_store().get_game_folder();
    let pick_folder = move |_| {
        spawn(async move {
            if let Some(path) = rfd::AsyncFileDialog::new().pick_folder().await {
                settings_store.write().set_game_folder(Some(path.path().to_owned()));
            }
        });
    };

    let path_display = if let Some(path) = game_folder.as_ref() {
        truncate_path(path, 40)
    } else {
        String::new()
    };

    let current_theme_label = match settings_store().get_theme() {
        AppTheme::Dark => "Dark Mode",
        AppTheme::Light => "Light Mode",
    };

    rsx! {
        div {
            style: "width: 250px; background-color: var(--bg-primary); padding: 20px; display: flex; flex-direction: column; gap: 10px; border-right: 1px solid var(--bg-tertiary); overflow: hidden;",

            h3 { style: "margin: 0; color: var(--text-primary); font-size: 16px;", "Hytale Mod Manager" }
            span { style: "font-size: 12px; color: var(--text-secondary);", "Curse Forge Client" }
            div { style: "height: 20px;" }

            SidebarBtn {
                label: "📦 Search Mods",
                active: active_tab() == SidebarTab::SearchMods,
                onclick: move |_| active_tab.set(SidebarTab::SearchMods)
            }

            SidebarBtn {
                label: "Installed",
                active: active_tab() == SidebarTab::Installed,
                onclick: move |_| active_tab.set(SidebarTab::Installed)
            }

            div { style: "flex: 1;" }

            span { style: "font-size: 12px; color: var(--text-secondary); display: block; margin-bottom: 5px;", "Theme" }
            ThemeDropdown {
                items: ["Dark Mode".to_string(), "Light Mode".to_string()],
                upwards: true,
                placeholder: "Theme",
                default_index: Some( settings_store().get_theme() as usize ),
                on_select: move |idx| {
                    if idx == 0 { settings_store.write().set_theme(AppTheme::Dark); }
                    else { settings_store.write().set_theme(AppTheme::Light); }
                },
            }

            button { class: "btn btn-brand", onclick: pick_folder, "📂 Set Game Folder" }
            button { class: "btn btn-neutral", onclick: move |_| on_open_api.call(()), "🔑 Set Api Key" }

            if let Some(path) = game_folder {
                div {
                    title: "{path.to_string_lossy()}",
                    style: "font-size: 10px; color: var(--brand-primary); text-align: center; white-space: nowrap; overflow: hidden;",
                    "{path_display}"
                }
            } else {
                div { style: "font-size: 10px; color: var(--danger); text-align: center;", "No folder set" }
            }
        }
    }
}

#[component]
fn SidebarBtn(label: &'static str, active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let active_class = if active { "active" } else { "" };
    rsx! {
        button {
            class: "btn btn-neutral {active_class}",
            style: "width: 100%;",
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}

