#[cfg(target_arch = "wasm32")]
use crate::interfaces::brick::BrickState;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::tutorial::TutorialViewState;

#[cfg(target_arch = "wasm32")]
pub fn selected_bricks(tutorial: &TutorialViewState, export_selection: &[bool]) -> Vec<BrickState> {
    tutorial
        .get_brick_state_list()
        .into_iter()
        .enumerate()
        .filter_map(|(index, brick)| {
            export_selection
                .get(index)
                .copied()
                .unwrap_or(false)
                .then_some(brick)
        })
        .collect()
}

#[cfg(target_arch = "wasm32")]
pub fn export_selected_bricks<F>(
    tutorial: &TutorialViewState,
    export_selection: &[bool],
    exporter: F,
) -> Result<(), String>
where
    F: FnOnce(&[BrickState]) -> Result<(), String>,
{
    let selected = selected_bricks(tutorial, export_selection);
    if selected.is_empty() {
        return Err("Export error: no bricks selected".to_string());
    }
    exporter(&selected)
}

#[cfg(target_arch = "wasm32")]
pub fn resized(len: usize) -> Vec<bool> {
    vec![false; len]
}

#[cfg(target_arch = "wasm32")]
pub fn toggled(current: &[bool], index: usize) -> Vec<bool> {
    let mut next = current.to_vec();
    if index < next.len() {
        next[index] = !next[index];
    }
    next
}

#[cfg(target_arch = "wasm32")]
pub fn all_selected(len: usize) -> Vec<bool> {
    vec![true; len]
}

#[cfg(target_arch = "wasm32")]
pub fn cleared(len: usize) -> Vec<bool> {
    vec![false; len]
}
