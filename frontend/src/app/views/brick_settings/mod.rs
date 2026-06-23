#[cfg(target_arch = "wasm32")]
mod colors_group;
#[cfg(target_arch = "wasm32")]
mod content_group;
#[cfg(target_arch = "wasm32")]
mod types_group;

#[cfg(target_arch = "wasm32")]
pub mod brick_settings_view;

#[cfg(target_arch = "wasm32")]
pub use brick_settings_view::BrickSettingsView;

#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_SETTINGS_EDITOR_GROUP_CLASS: &str = "w-full";
#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_SETTINGS_EDITOR_GROUP_CONTENT: &str = "min-h-0 flex-1 overflow-y-auto";
#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_TYPES_SCROLL_WRAP: &str = "min-h-0 h-full overflow-y-auto max-[900px]:overflow-x-auto max-[900px]:overflow-y-hidden max-[900px]:pb-1";
#[cfg(target_arch = "wasm32")]
pub(crate) const CONTENT_GROUP_CLASS: &str = "flex-none overflow-hidden";
#[cfg(target_arch = "wasm32")]
pub(crate) const CONTENT_GROUP_CONTENT_CLASS: &str =
    "flex-none overflow-hidden max-[900px]:items-stretch";
#[cfg(target_arch = "wasm32")]
pub(crate) const CONTENT_GROUP_SHELL: &str = "relative flex min-w-0 flex-1 w-full";
#[cfg(target_arch = "wasm32")]
pub(crate) const CONTENT_GROUP_TEXTAREA: &str = "min-h-[8lh] w-full flex-1 resize-none rounded-[var(--app-radius)] border border-app-border bg-app-surface-raised px-2.5 py-1.5 text-[14px] text-app-text outline-none focus:border-app-accent max-[900px]:max-h-[6lh] max-[900px]:min-h-[4lh]";
#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_SETTINGS_ROOT: &str =
    "flex min-h-0 min-w-0 flex-1 flex-col gap-app-gap overflow-hidden";
#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_SETTINGS_ROW: &str =
    "flex min-h-0 flex-1 gap-app-gap overflow-hidden max-[900px]:flex-col";
#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_SETTINGS_COLUMN: &str = "flex min-h-0 min-w-0 flex-1 overflow-hidden";

#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_VIEW_SCROLL_WRAP: &str =
    "max-[900px]:overflow-x-auto max-[900px]:overflow-y-hidden max-[900px]:pb-1";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_VIEW_SHELL: &str = "relative";

#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_ADD_BRICK: &str = include_str!("../../../res/addBrick.svg");
