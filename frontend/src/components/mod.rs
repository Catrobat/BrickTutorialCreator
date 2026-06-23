pub mod brick_view;
pub mod card;
pub mod cards;
pub mod context_menu;
pub mod custom_color_modal;
pub mod drag_drop;
pub mod editor_group;
pub mod export_bar;
pub mod icon_button;
pub mod modal;
pub mod sidebar;

#[cfg(target_arch = "wasm32")]
pub(crate) const EDITOR_GROUP_ROOT: &str = "flex min-h-0 flex-1 flex-col overflow-hidden rounded-[var(--app-radius)] border border-app-border bg-app-surface";
#[cfg(target_arch = "wasm32")]
pub(crate) const EDITOR_GROUP_HEADER: &str = "flex shrink-0 items-center justify-between gap-2 border-b border-app-border bg-app-surface-raised px-3 py-2";
#[cfg(target_arch = "wasm32")]
pub(crate) const EDITOR_GROUP_CONTENT: &str =
    "flex min-h-0 flex-1 flex-col overflow-hidden p-app-gap";
#[cfg(target_arch = "wasm32")]
pub(crate) const EDITOR_GROUP_TITLE: &str =
    "text-[12px] font-semibold tracking-[0.06em] text-app-text-muted uppercase";
#[cfg(target_arch = "wasm32")]
pub(crate) const EDITOR_GROUP_ACTIONS: &str = "ml-auto flex items-center gap-1.5";

#[cfg(target_arch = "wasm32")]
pub(crate) const CARD_ROOT: &str = "flex h-16 w-[90px] shrink-0 flex-col overflow-hidden rounded-[var(--app-radius)] border border-app-border bg-app-surface-raised px-[10px] py-2 max-[900px]:h-auto max-[900px]:min-h-16 max-[900px]:min-w-[70px] max-[900px]:max-w-24 max-[900px]:w-auto max-[900px]:px-2 max-[900px]:py-1.5";
#[cfg(target_arch = "wasm32")]
pub(crate) const CARD_SELECTABLE: &str = "cursor-pointer select-none hover:border-app-accent";
#[cfg(target_arch = "wasm32")]
pub(crate) const CARD_SELECTED: &str =
    "border-app-accent bg-[color-mix(in_srgb,var(--app-accent)_15%,var(--app-surface-raised))]";
#[cfg(target_arch = "wasm32")]
pub(crate) const CARD_MOBILE_CIRCLE: &str = "max-[900px]:h-[30px] max-[900px]:w-[30px] max-[900px]:min-h-[30px] max-[900px]:min-w-[30px] max-[900px]:max-w-[30px] max-[900px]:items-center max-[900px]:justify-center max-[900px]:rounded-full max-[900px]:border-2 max-[900px]:bg-transparent max-[900px]:p-0";
#[cfg(target_arch = "wasm32")]
pub(crate) const CARD_MOBILE_CIRCLE_SELECTED: &str = "max-[900px]:bg-[color-mix(in_srgb,var(--app-accent)_18%,var(--app-surface-raised))] max-[900px]:shadow-[0_0_0_2px_color-mix(in_srgb,var(--app-accent)_30%,transparent)]";
#[cfg(target_arch = "wasm32")]
pub(crate) const CARD_TITLE: &str = "mb-1 overflow-hidden text-ellipsis whitespace-nowrap text-[12px] font-semibold text-app-text max-[900px]:mb-0.5 max-[900px]:text-[11px]";
#[cfg(target_arch = "wasm32")]
pub(crate) const CARD_CONTENT: &str = "flex flex-col gap-1";

#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_BUTTON: &str = "inline-flex h-9 w-9 shrink-0 items-center justify-center rounded-[var(--app-radius)] border border-app-border bg-app-surface-raised p-0 text-app-text transition-colors hover:bg-app-border disabled:cursor-not-allowed disabled:opacity-40 disabled:hover:bg-app-surface-raised [&_svg]:pointer-events-none [&_.icon-spinner]:animate-spin";

#[cfg(target_arch = "wasm32")]
pub(crate) const MODAL_OVERLAY: &str =
    "fixed inset-0 z-[1000] flex items-center justify-center bg-black/55 p-6";
#[cfg(target_arch = "wasm32")]
pub(crate) const MODAL_ROOT: &str = "flex h-[min(720px,92vh)] w-[min(980px,96vw)] flex-col overflow-hidden rounded-[var(--app-radius)] border border-app-border bg-app-surface";
#[cfg(target_arch = "wasm32")]
pub(crate) const MODAL_HEADER: &str = "flex shrink-0 items-center justify-between gap-app-gap border-b border-app-border bg-app-surface-raised px-3 py-2.5";
#[cfg(target_arch = "wasm32")]
pub(crate) const MODAL_TITLE_ROW: &str = "flex min-w-0 items-baseline gap-2.5";
#[cfg(target_arch = "wasm32")]
pub(crate) const MODAL_TITLE: &str = "font-bold";
#[cfg(target_arch = "wasm32")]
pub(crate) const MODAL_HINT: &str = "whitespace-nowrap text-[12px] font-normal text-app-text-muted";
#[cfg(target_arch = "wasm32")]
pub(crate) const MODAL_CLOSE_BUTTON: &str = "inline-flex h-8 w-8 items-center justify-center rounded-[var(--app-radius)] border border-app-border bg-app-surface p-0 text-app-text transition-colors hover:bg-app-border";
#[cfg(target_arch = "wasm32")]
pub(crate) const MODAL_BODY: &str =
    "flex min-h-0 flex-1 flex-col gap-app-gap overflow-auto p-app-gap";

#[cfg(target_arch = "wasm32")]
pub(crate) const SIDEBAR_ROOT: &str = "flex min-w-0 w-[var(--app-toggle-width)] shrink-0 flex-row overflow-hidden border-l border-app-border bg-app-surface transition-[width] duration-200 ease-in-out max-[900px]:fixed max-[900px]:right-0 max-[900px]:top-0 max-[900px]:bottom-0 max-[900px]:z-20 max-[900px]:h-dvh max-[900px]:w-[var(--app-toggle-width)] max-[900px]:shadow-[-8px_0_16px_rgba(0,0,0,0.25)]";
#[cfg(target_arch = "wasm32")]
pub(crate) const SIDEBAR_ROOT_OPEN: &str =
    "w-[calc(var(--app-sidebar-width)+var(--app-toggle-width))] max-[900px]:w-screen";
#[cfg(target_arch = "wasm32")]
pub(crate) const SIDEBAR_TOGGLE_WRAP: &str = "flex h-full basis-[var(--app-toggle-width)] flex-col border-r border-app-border max-[900px]:absolute max-[900px]:left-0 max-[900px]:top-0 max-[900px]:bottom-0 max-[900px]:z-[2] max-[900px]:bg-app-surface";
#[cfg(target_arch = "wasm32")]
pub(crate) const SIDEBAR_TOGGLE_BUTTON: &str = "flex flex-1 items-center justify-center bg-transparent px-0 py-2 text-app-text transition-colors hover:bg-app-surface-raised";
#[cfg(target_arch = "wasm32")]
pub(crate) const SIDEBAR_CONTENT: &str = "flex min-w-0 flex-1 flex-col overflow-auto w-[var(--app-sidebar-width)] p-app-gap max-[900px]:w-screen max-[900px]:pl-[calc(var(--app-gap)+var(--app-toggle-width))]";

#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_VIEW_IMAGE: &str = "block";

#[cfg(target_arch = "wasm32")]
pub(crate) const CONTEXT_MENU: &str = "fixed z-[200] flex min-w-40 flex-col gap-1 rounded-[var(--app-radius)] border border-app-border bg-app-surface p-1.5 shadow-[0_12px_24px_rgba(0,0,0,0.18)]";
#[cfg(target_arch = "wasm32")]
pub(crate) const CONTEXT_MENU_ITEM: &str = "w-full cursor-pointer rounded-[calc(var(--app-radius)-2px)] bg-transparent px-2.5 py-2 text-left text-app-text transition-colors hover:bg-app-surface-raised disabled:cursor-not-allowed disabled:opacity-45 disabled:hover:bg-transparent";

#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_ROOT: &str = "h-auto max-h-[92vh] w-[min(720px,96vw)]";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_BODY: &str = "gap-4";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_GRID: &str =
    "grid grid-cols-[minmax(0,1.3fr)_minmax(220px,0.9fr)] gap-4 max-[900px]:grid-cols-1";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_FORM: &str = "flex flex-col gap-3";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_FIELD: &str = "flex flex-col gap-1.5";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_FIELD_ROW: &str = "grid grid-cols-2 gap-3 max-[640px]:grid-cols-1";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_LABEL: &str =
    "text-[12px] font-semibold tracking-[0.04em] text-app-text-muted uppercase";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_CHECKBOX_ROW: &str = "flex items-start gap-2.5 rounded-[var(--app-radius)] border border-app-border bg-app-surface-raised px-3 py-2.5";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_CHECKBOX: &str =
    "mt-0.5 h-4 w-4 cursor-pointer accent-[var(--app-accent)]";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_CHECKBOX_TEXT: &str = "flex flex-col gap-0.5";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_CHECKBOX_TITLE: &str = "text-[13px] font-semibold text-app-text";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_CHECKBOX_HINT: &str = "text-[12px] text-app-text-muted";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_TEXT_INPUT: &str = "w-full rounded-[var(--app-radius)] border border-app-border bg-app-surface-raised px-3 py-2 text-[14px] text-app-text outline-none transition-colors focus:border-app-accent";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_COLOR_INPUT_WRAP: &str = "flex items-center gap-2 rounded-[var(--app-radius)] border border-app-border bg-app-surface-raised px-2 py-2";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_COLOR_INPUT: &str =
    "h-9 w-12 cursor-pointer rounded border border-app-border bg-transparent p-0";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_COLOR_VALUE: &str = "font-mono text-[12px] text-app-text-muted";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_PREVIEW: &str = "flex flex-col gap-3 rounded-[var(--app-radius)] border border-app-border bg-app-surface-raised p-3";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_PREVIEW_TITLE: &str =
    "text-[12px] font-semibold tracking-[0.04em] text-app-text-muted uppercase";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_PREVIEW_CARD: &str = "flex min-h-28 flex-col justify-between rounded-[var(--app-radius)] border border-app-border bg-app-surface p-3";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_PREVIEW_NAME: &str = "text-[14px] font-semibold text-app-text";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_PREVIEW_SWATCH_ROW: &str = "flex items-stretch gap-2";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_PREVIEW_SWATCH: &str =
    "flex min-h-14 flex-1 items-center justify-center rounded-[6px] text-sm font-semibold";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_ERROR: &str = "rounded-[var(--app-radius)] border border-red-500/40 bg-red-500/10 px-3 py-2 text-[12px] text-red-200";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_ACTIONS: &str = "flex items-center justify-end gap-2";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_BUTTON: &str = "rounded-[var(--app-radius)] border border-app-border bg-app-surface px-3 py-2 text-app-text transition-colors hover:bg-app-border disabled:cursor-not-allowed disabled:opacity-50";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_MODAL_BUTTON_PRIMARY: &str = "border-app-accent bg-app-accent text-white hover:bg-[color-mix(in_srgb,var(--app-accent)_85%,black)]";

#[cfg(target_arch = "wasm32")]
pub(crate) const TUTORIAL_EDITOR_EXPORT_BAR: &str = "flex flex-wrap items-center gap-1.5 rounded-[var(--app-radius)] border border-app-border bg-app-surface-raised p-2";
#[cfg(target_arch = "wasm32")]
pub(crate) const TUTORIAL_EDITOR_EXPORT_STATUS: &str = "mr-1.5 text-[12px] text-app-text-muted";
#[cfg(target_arch = "wasm32")]
pub(crate) const TUTORIAL_EDITOR_EXPORT_BUTTON: &str = "rounded-[var(--app-radius)] border border-app-border bg-app-surface px-2 py-1 text-app-text transition-colors hover:bg-app-border disabled:cursor-not-allowed disabled:opacity-50";

#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_CLOSE: &str = include_str!("../res/close.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_CHEVRON_RIGHT: &str = include_str!("../res/chevron_right.svg");
#[cfg(target_arch = "wasm32")]
pub(crate) const ICON_CHEVRON_LEFT: &str = include_str!("../res/chevron_left.svg");
