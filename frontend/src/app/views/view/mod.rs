pub mod brick_preview_view;
pub mod brick_type_view;
pub mod color_view;
pub mod tutorial_edit_view;
pub mod tutorial_preview_view;
pub mod tutorial_settings_view;

#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_PREVIEW_GRID: &str = "grid flex-none max-[900px]:max-h-[45vh]";
#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_PREVIEW_IMAGE: &str =
    "col-start-1 row-start-1 h-full w-full overflow-hidden";
#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_PREVIEW_Y_SLIDER: &str =
    "col-start-2 row-start-1 min-h-0 min-w-0 flex-1 [direction:ltr] [writing-mode:vertical-lr]";
#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_PREVIEW_X_SLIDER: &str = "col-start-1 row-start-2 min-h-0 min-w-0 flex-1";

#[cfg(target_arch = "wasm32")]
pub(crate) const TUTORIAL_EMPTY_STATE: &str = "py-6 text-center text-[12px] text-app-text-muted";
#[cfg(target_arch = "wasm32")]
pub(crate) const TUTORIAL_LIST: &str = "flex min-h-0 flex-1 flex-col overflow-y-auto p-[2px]";
#[cfg(target_arch = "wasm32")]
pub(crate) const TUTORIAL_LIST_ITEM: &str = "relative mx-[-4px] my-[2%] cursor-pointer rounded-[var(--app-radius)] border-2 border-transparent px-1 py-0 transition-colors select-none hover:border-app-border [&+&]:mt-[-2.5%] [&[draggable='true']]:cursor-grab active:[&[draggable='true']]:cursor-grabbing";
#[cfg(target_arch = "wasm32")]
pub(crate) const TUTORIAL_LIST_ITEM_SELECTED: &str = "border-app-accent";
#[cfg(target_arch = "wasm32")]
pub(crate) const TUTORIAL_ROW: &str = "flex items-center gap-1.5";
#[cfg(target_arch = "wasm32")]
pub(crate) const TUTORIAL_EXPORT_LABEL: &str = "flex items-center justify-center px-1";
#[cfg(target_arch = "wasm32")]
pub(crate) const TUTORIAL_EXPORT_CHECKBOX: &str = "h-4 w-4 cursor-pointer";
#[cfg(target_arch = "wasm32")]
pub(crate) const TUTORIAL_MOVE_BUTTONS: &str = "flex flex-col justify-center gap-0.5";
#[cfg(target_arch = "wasm32")]
pub(crate) const TUTORIAL_MOVE_BUTTON: &str = "inline-flex items-center justify-center rounded-[var(--app-radius)] border border-app-border bg-app-bg px-1.5 py-0.5 text-app-text transition-colors hover:border-app-accent disabled:cursor-not-allowed disabled:opacity-40 [&_svg:not([class*='size-'])]:size-4";
#[cfg(target_arch = "wasm32")]
pub(crate) const TUTORIAL_BRICK_VIEW: &str = "flex-1";

#[cfg(target_arch = "wasm32")]
pub(crate) const TUTORIAL_SETTINGS_ACTIONS: &str = "flex flex-row flex-wrap gap-1.5";

#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_VIEW_GRID: &str = "flex flex-wrap gap-1.5 max-[900px]:h-auto max-[900px]:w-max max-[900px]:flex-nowrap max-[900px]:items-center max-[900px]:gap-2";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_ADD_BUTTON: &str = "flex h-16 w-[90px] shrink-0 flex-col items-center justify-center gap-1 rounded-[var(--app-radius)] border border-dashed border-app-border bg-app-surface-raised px-[10px] py-2 text-app-text transition-colors hover:border-app-accent hover:bg-app-surface max-[900px]:h-[30px] max-[900px]:w-[30px] max-[900px]:min-h-[30px] max-[900px]:min-w-[30px] max-[900px]:rounded-full max-[900px]:border-2 max-[900px]:p-0";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_ADD_BUTTON_ICON: &str = "flex h-8 w-8 items-center justify-center rounded-full bg-app-surface text-[24px] leading-none max-[900px]:h-auto max-[900px]:w-auto max-[900px]:bg-transparent";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_ADD_BUTTON_LABEL: &str =
    "text-center text-[11px] font-semibold leading-tight max-[900px]:hidden";

#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_TYPE_VIEW_GRID: &str =
    "flex flex-wrap gap-2 brick-type-mobile-grid max-[900px]:w-max max-[900px]:flex-nowrap";

#[cfg(target_arch = "wasm32")]
pub(crate) const TUTORIAL_PREVIEW_WRAP: &str = "min-h-0 flex-1 overflow-y-auto";
#[cfg(target_arch = "wasm32")]
pub(crate) const TUTORIAL_PREVIEW_IMAGE: &str = "block h-auto w-full rounded-[var(--app-radius)]";

#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_ADD: &str = include_str!("../../../res/add.svg");
