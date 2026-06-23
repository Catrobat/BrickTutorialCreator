pub mod brick_catalog_modal;
pub mod brick_settings;
pub mod view;

#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_CATALOG_GROUPS: &str = "flex flex-col gap-app-gap";
#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_CATALOG_GROUP_WRAP: &str = "flex-none";
#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_CATALOG_EDITOR_GROUP_CLASS: &str = "flex-none";
#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_CATALOG_EDITOR_GROUP_CONTENT: &str = "flex-none overflow-visible";
#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_CATALOG_GRID: &str = "flex flex-wrap gap-2.5";
#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_CATALOG_CARD: &str = "h-[92px] w-[120px]";
#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_CATALOG_PREVIEW_FRAME: &str = "flex h-10 items-center justify-center overflow-hidden rounded-[4px] border border-app-border bg-app-surface";
#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_CATALOG_PREVIEW_IMAGE: &str = "block h-full w-full object-contain";
#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_CATALOG_INVALID: &str = "text-[11px] text-app-text-muted";
