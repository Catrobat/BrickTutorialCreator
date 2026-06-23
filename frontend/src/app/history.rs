#[cfg(target_arch = "wasm32")]
use crate::interfaces::brick::BrickState;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::tutorial::TutorialViewState;

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Default, PartialEq)]
pub struct AppSnapshot {
    pub brick: BrickState,
    pub tutorial: TutorialViewState,
}

#[cfg(target_arch = "wasm32")]
pub fn next_history(
    history: &[AppSnapshot],
    current_index: usize,
    snapshot: &AppSnapshot,
) -> Option<(Vec<AppSnapshot>, usize)> {
    if history
        .get(current_index)
        .map(|entry| entry == snapshot)
        .unwrap_or(false)
    {
        return None;
    }

    let mut next = history.to_vec();
    next.truncate(current_index + 1);
    next.push(snapshot.clone());
    let next_index = current_index + 1;
    Some((next, next_index))
}

#[cfg(target_arch = "wasm32")]
pub fn pending_restore_count(
    current_brick: &BrickState,
    current_tutorial: &TutorialViewState,
    target: &AppSnapshot,
) -> usize {
    usize::from(current_brick != &target.brick) + usize::from(current_tutorial != &target.tutorial)
}
