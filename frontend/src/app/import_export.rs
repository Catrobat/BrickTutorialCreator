#[cfg(target_arch = "wasm32")]
use crate::app::browser;
#[cfg(target_arch = "wasm32")]
use crate::app::export_selection::export_selected_bricks;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::brick::BrickState;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::tutorial::{
    TutorialAction, TutorialViewState, tutorial_from_states, tutorial_png_bytes,
};
#[cfg(target_arch = "wasm32")]
use shared::tutorial::Tutorial;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
pub const ALL_BRICKS_TILE_WIDTH: u32 = 192;
#[cfg(target_arch = "wasm32")]
pub const SELECTED_TUTORIAL_WIDTH: u32 = 1920;

#[cfg(target_arch = "wasm32")]
pub fn import_json_text(
    text: String,
    tutorial_dispatcher: &UseReducerDispatcher<TutorialViewState>,
) {
    if Tutorial::from_json(&text).is_ok() {
        tutorial_dispatcher.dispatch(TutorialAction::LoadJson(text));
        return;
    }
    if let Ok(brick) = BrickState::from_json(&text) {
        tutorial_dispatcher.dispatch(TutorialAction::InsertAfterSelected(vec![brick]));
        return;
    }
    web_sys::console::error_1(&"Import error: unsupported JSON (not a brick or tutorial)".into());
}

#[cfg(target_arch = "wasm32")]
pub fn export_selected_json(
    tutorial: &TutorialViewState,
    export_selection: &[bool],
) -> Result<(), String> {
    export_selected_bricks(tutorial, export_selection, |selected| {
        let tutorial = tutorial_from_states(selected, "Exported tutorial");
        let json = tutorial.to_json();
        browser::download_json(&json, "tutorial.json").map_err(|e| format!("Export error: {e}"))
    })
}

#[cfg(target_arch = "wasm32")]
pub fn export_selected_png(
    tutorial: &TutorialViewState,
    export_selection: &[bool],
) -> Result<(), String> {
    export_selected_bricks(tutorial, export_selection, |selected| {
        let data = tutorial_png_bytes(selected, SELECTED_TUTORIAL_WIDTH)
            .map_err(|e| format!("PNG render error: {e}"))?;
        browser::download_png(&data, "tutorial.png").map_err(|e| format!("PNG error: {e}"))
    })
}

#[cfg(target_arch = "wasm32")]
pub fn export_selected_svg(
    tutorial: &TutorialViewState,
    export_selection: &[bool],
) -> Result<(), String> {
    export_selected_bricks(tutorial, export_selection, |selected| {
        let exported = tutorial_from_states(selected, "Exported tutorial");
        let svg = shared::export::tutorial_svg(&exported)
            .map_err(|e| format!("SVG render error: {e}"))?;
        browser::download_svg(&svg, "tutorial.svg").map_err(|e| format!("SVG error: {e}"))
    })
}
