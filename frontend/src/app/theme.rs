#[cfg(target_arch = "wasm32")]
use super::THEME_STORAGE_KEY;

#[cfg(target_arch = "wasm32")]
fn document() -> web_sys::Document {
    gloo::utils::document()
}

#[cfg(target_arch = "wasm32")]
pub fn load_saved_theme() -> bool {
    gloo::utils::window()
        .local_storage()
        .ok()
        .flatten()
        .and_then(|s| s.get_item(THEME_STORAGE_KEY).ok().flatten())
        .map(|v| v == "light")
        .unwrap_or(false)
}

#[cfg(target_arch = "wasm32")]
pub fn apply_theme(light: bool) {
    if let Some(el) = document().document_element() {
        if light {
            let _ = el.set_attribute("data-theme", "light");
        } else {
            let _ = el.remove_attribute("data-theme");
        }
    }
    if let Ok(Some(storage)) = gloo::utils::window().local_storage() {
        let _ = storage.set_item(THEME_STORAGE_KEY, if light { "light" } else { "dark" });
    }
}
