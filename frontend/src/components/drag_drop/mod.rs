pub mod drag_drop_item;
pub mod drag_drop_list;

#[cfg(target_arch = "wasm32")]
pub(crate) const DRAG_DROP_LIST: &str = "drag-drop-list";
#[cfg(target_arch = "wasm32")]
pub(crate) const DRAG_DROP_LIST_ITEM: &str = "drag-drop-list__item";
#[cfg(target_arch = "wasm32")]
pub(crate) const DRAG_DROP_LIST_ITEM_SELECTED: &str = "drag-drop-list__item--selected";
