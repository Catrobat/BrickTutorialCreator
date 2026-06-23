#[cfg(target_arch = "wasm32")]
use shared::color::ColorScheme;

#[cfg(target_arch = "wasm32")]
const CUSTOM_COLOR_STORAGE_KEY: &str = "brickcreator.custom_colors";

#[cfg(target_arch = "wasm32")]
pub fn load_saved_custom_colors() -> Vec<ColorScheme> {
    gloo::utils::window()
        .local_storage()
        .ok()
        .flatten()
        .and_then(|storage| storage.get_item(CUSTOM_COLOR_STORAGE_KEY).ok().flatten())
        .and_then(|value| serde_json::from_str::<Vec<ColorScheme>>(&value).ok())
        .unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
pub fn persist_saved_custom_colors(colors: &[ColorScheme]) {
    let Ok(Some(storage)) = gloo::utils::window().local_storage() else {
        return;
    };

    if colors.is_empty() {
        let _ = storage.remove_item(CUSTOM_COLOR_STORAGE_KEY);
        return;
    }

    if let Ok(value) = serde_json::to_string(colors) {
        let _ = storage.set_item(CUSTOM_COLOR_STORAGE_KEY, &value);
    }
}
