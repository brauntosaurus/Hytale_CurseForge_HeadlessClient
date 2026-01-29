use dioxus::prelude::*;

#[component]
pub fn ThemeDropdown(
    #[props(into)]
    items: Vec<String>,
    on_select: EventHandler<usize>,
    upwards: bool,
    default_index: Option<usize>,
    placeholder: String,
) -> Element {
    let mut is_open = use_signal(|| false);

    let mut selected_index = use_signal(|| default_index);
    let mut last_external_prop = use_signal(|| default_index);

    use_effect(move || {
        if default_index != last_external_prop() {
            selected_index.set(default_index);
            last_external_prop.set(default_index);
        }
    });

    let current_label = match selected_index() {
        Some(i) if i < items.len() => items[i].clone(),
        _ => placeholder.clone(),
    };

    let arrow = if is_open() { "▲" } else { "▼" };
    let position_style = if upwards { "bottom: 110%; top: auto;" } else { "top: 110%; bottom: auto;" };
    let display_mode = if is_open() { "block" } else { "none" };

    rsx! {
        div {
            style: "position: relative;",

            button {
                class: "btn",
                style: "width: 100%; min-height: 40px; background-color: var(--input-bg); border: 1px solid var(--border-color); color: var(--text-primary); justify-content: space-between; display: flex; align-items: center; padding: 10px;",
                onclick: move |_| is_open.set(!is_open()),
                span { "{current_label}" }
                span { style: "font-size: 10px; margin-left: 10px;", "{arrow}" }
            }
            div {
                style: "
                    display: {display_mode};
                    position: absolute;
                    left: 0;
                    width: 100%;

                    z-index: 10;
                    {position_style}
                ",
                div {
                    style: "
                    background-color: var(--input-bg);
                    border: 1px solid var(--border-color);

                    border-radius: 4px;
                    ",

                    for (i, item) in items.iter().enumerate() {
                        div {
                            class: "dropdown-item",
                            style: "padding: 10px; cursor: pointer; color: var(--text-secondary); transition: 0.2s;",
                            onclick: move |_| {
                                selected_index.set(Some(i));
                                is_open.set(false);
                                on_select.call(i);
                            },
                            "{item}"
                        }
                    }
                }

            }
        }
        style {{ r#"
            .dropdown-item:hover {
                background-color: var(--brand-primary);
                color: white !important;
            }
        "# }}
    }
}