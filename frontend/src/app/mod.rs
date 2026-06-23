pub mod application;
pub mod browser;
pub mod editors;
pub mod export_selection;
pub mod history;
pub mod import_export;
pub mod theme;
pub mod toolbar;
pub mod views;

#[cfg(target_arch = "wasm32")]
pub(crate) const APP_ROOT: &str = "flex h-screen flex-col overflow-hidden max-[900px]:relative";
#[cfg(target_arch = "wasm32")]
pub(crate) const APP_TOOLBAR: &str = "relative z-50 flex w-full items-center justify-between gap-2.5 overflow-visible border-b border-app-border bg-app-surface px-app-gap py-2.5 max-[900px]:order-2 max-[900px]:sticky max-[900px]:bottom-0 max-[900px]:z-[60] max-[900px]:justify-center max-[900px]:border-t max-[900px]:border-b-0 max-[900px]:bg-app-surface-raised max-[900px]:pb-[calc(10px+env(safe-area-inset-bottom))]";
#[cfg(target_arch = "wasm32")]
pub(crate) const APP_TOOLBAR_GROUP: &str = "flex items-center gap-2.5 overflow-visible";
#[cfg(target_arch = "wasm32")]
pub(crate) const TOOLBAR_MENU: &str = "toolbar-menu";
#[cfg(target_arch = "wasm32")]
pub(crate) const TOOLBAR_MENU_DROPDOWN: &str = "toolbar-menu__dropdown";
#[cfg(target_arch = "wasm32")]
pub(crate) const TOOLBAR_MENU_ITEM: &str = "toolbar-menu__item";
#[cfg(target_arch = "wasm32")]
pub(crate) const TOOLBAR_MENU_ICON: &str = "toolbar-menu__icon";
#[cfg(target_arch = "wasm32")]
pub(crate) const APP_MAIN: &str = "flex min-h-0 flex-1 overflow-hidden max-[900px]:order-1 max-[900px]:h-[calc(100dvh-56px)] max-[900px]:pb-[56px]";
#[cfg(target_arch = "wasm32")]
pub(crate) const APP_EDITOR_WRAP: &str = "flex min-w-0 flex-1 flex-col overflow-auto p-app-gap max-[900px]:h-full max-[900px]:flex-auto max-[900px]:pr-[calc(var(--app-gap)+var(--app-toggle-width))]";

#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_UPLOAD: &str = include_str!("../res/upload.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_DOWNLOAD: &str = include_str!("../res/download.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_EXPORT_ALL_BRICKS: &str = include_str!("../res/zipfolder.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_EXPORT_NINEPATCH: &str = include_str!("../res/ninepatch.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_LOADING: &str = include_str!("../res/loading.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_MENU: &str = include_str!("../res/menu.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_UNDO: &str = include_str!("../res/undo.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_REDO: &str = include_str!("../res/redo.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_RESET: &str = include_str!("../res/reset.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_HELP: &str = include_str!("../res/help.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_ABOUT: &str = include_str!("../res/about.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_ADD_CUSTOM: &str = include_str!("../res/add.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_ADD_BRICK: &str = include_str!("../res/addBrick.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_DELETE: &str = include_str!("../res/delete.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_EDIT: &str = include_str!("../res/edit.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_PREVIEW: &str = include_str!("../res/preview.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_EDIT_SQUARE: &str = include_str!("../res/editsquare.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_BRICK_CATALOG: &str = include_str!("../res/brickcatalog.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_MOVE: &str = include_str!("../res/swap_vert.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_CHEVRON_RIGHT: &str = include_str!("../res/chevron_right.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_MOVE_UP: &str = include_str!("../res/keyboard_arrow_up.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_MOVE_DOWN: &str = include_str!("../res/keyboard_arrow_down.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_SUN: &str = include_str!("../res/sun.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_MOON: &str = include_str!("../res/moon.svg");

#[cfg(target_arch = "wasm32")]
pub(crate) const HELP_DOCS_URL: &str = "https://catrobat.org/docs/brickdocumentation/";
#[cfg(target_arch = "wasm32")]
pub(crate) const HELP_CONTACT_URL: &str = "https://developer.catrobat.org/pages/legal/imprint/";
#[cfg(target_arch = "wasm32")]
pub(crate) const ABOUT_MESSAGE: &str = "About BrickCreator\n\nBrickCreator is a website built by and for Catrobat Pocket Code users, especially educators who teach Pocket Code.\n\nIt allows users to create PNG, SVG, and JSON files that can be directly used in presentations, teaching materials, and tutorials.\n\nCatrobat is an open-source platform with contributors from all over the world. For more information, visit: https://catrobat.org/about";
#[cfg(target_arch = "wasm32")]
pub(crate) const THEME_STORAGE_KEY: &str = "theme";
