pub mod brick_editor;
pub mod tutorial_editor;

#[cfg(target_arch = "wasm32")]
pub(crate) const TUTORIAL_EDITOR_CONTENT: &str = "flex min-h-0 min-w-0 flex-1 flex-col gap-app-gap";
