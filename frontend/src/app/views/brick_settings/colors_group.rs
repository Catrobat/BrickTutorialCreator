#[cfg(target_arch = "wasm32")]
use crate::app::views::brick_settings as style;
#[cfg(target_arch = "wasm32")]
use crate::app::views::view::color_view::ColorView;
#[cfg(target_arch = "wasm32")]
use crate::components::context_menu::{ContextMenu, ContextMenuItem};
#[cfg(target_arch = "wasm32")]
use crate::components::custom_color_modal::CustomColorModal;
#[cfg(target_arch = "wasm32")]
use crate::components::editor_group::EditorGroup;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::color::{
    BrickColorModel, ColorModel, CustomColorDraft, CustomColorEditState,
};
#[cfg(target_arch = "wasm32")]
use crate::interfaces::color_storage::{load_saved_custom_colors, persist_saved_custom_colors};
#[cfg(target_arch = "wasm32")]
use shared::color::ColorScheme;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct ColorsGroupProps {
    pub selected_name: String,
    pub on_select: Callback<ColorScheme>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(ColorsGroup)]
pub fn colors_group(props: &ColorsGroupProps) -> Html {
    let model = use_state(|| BrickColorModel::with_custom_colors(load_saved_custom_colors()));
    let saved_custom_colors = use_state(load_saved_custom_colors);
    let modal_open = use_state(|| false);
    let draft = use_state(CustomColorDraft::default);
    let editing_color = use_state(|| Option::<CustomColorEditState>::None);
    let error_message = use_state(|| Option::<String>::None);
    let context_menu = use_state(|| Option::<(String, i32, i32)>::None);

    let colors = (*model).all_colors();
    let custom_color_names = colors
        .iter()
        .map(|color| color.name.clone())
        .filter(|name| {
            !model
                .default_colors()
                .iter()
                .any(|entry| entry.name == *name)
        })
        .collect::<Vec<_>>();

    let open_modal = {
        let modal_open = modal_open.clone();
        let draft = draft.clone();
        let editing_color = editing_color.clone();
        let error_message = error_message.clone();
        let context_menu = context_menu.clone();
        Callback::from(move |_| {
            draft.set(CustomColorDraft::default());
            editing_color.set(None);
            error_message.set(None);
            context_menu.set(None);
            modal_open.set(true);
        })
    };

    let close_modal = {
        let modal_open = modal_open.clone();
        let editing_color = editing_color.clone();
        let error_message = error_message.clone();
        let context_menu = context_menu.clone();
        Callback::from(move |_| {
            editing_color.set(None);
            error_message.set(None);
            context_menu.set(None);
            modal_open.set(false);
        })
    };

    let close_context_menu = {
        let context_menu = context_menu.clone();
        Callback::from(move |_e: MouseEvent| context_menu.set(None))
    };

    let on_draft_change = {
        let draft = draft.clone();
        Callback::from(move |next: CustomColorDraft| draft.set(next))
    };

    let on_save = {
        let model = model.clone();
        let saved_custom_colors = saved_custom_colors.clone();
        let draft = draft.clone();
        let editing_color = editing_color.clone();
        let error_message = error_message.clone();
        let modal_open = modal_open.clone();
        let context_menu = context_menu.clone();
        let on_select = props.on_select.clone();
        Callback::from(move |_| {
            let new_color = (*draft).to_scheme();
            if new_color.name.is_empty() {
                error_message.set(Some(
                    "Give the custom color a name before saving.".to_string(),
                ));
                return;
            }

            let mut next_model = (*model).clone();
            let mut next_saved = (*saved_custom_colors).clone();

            if let Some(editing) = &*editing_color {
                if !next_model.replace_custom_color(&editing.original_name, new_color.clone()) {
                    error_message.set(Some(
                        "That color name already exists. Pick a unique name.".to_string(),
                    ));
                    return;
                }

                let mut updated_saved = false;
                let was_saved = next_saved
                    .iter()
                    .any(|entry| entry.name == editing.original_name);

                if draft.save_for_later || was_saved {
                    for saved in &mut next_saved {
                        if saved.name == editing.original_name {
                            *saved = new_color.clone();
                            updated_saved = true;
                        }
                    }
                }

                if !draft.save_for_later {
                    let before = next_saved.len();
                    next_saved.retain(|entry| entry.name != editing.original_name);
                    updated_saved |= next_saved.len() != before;
                } else if !updated_saved {
                    next_saved.push(new_color.clone());
                    updated_saved = true;
                }

                if updated_saved {
                    persist_saved_custom_colors(&next_saved);
                    saved_custom_colors.set(next_saved);
                }
            } else {
                if !next_model.add_custom_color(new_color.clone()) {
                    error_message.set(Some(
                        "That color name already exists. Pick a unique name.".to_string(),
                    ));
                    return;
                }

                if draft.save_for_later {
                    next_saved.push(new_color.clone());
                    persist_saved_custom_colors(&next_saved);
                    saved_custom_colors.set(next_saved);
                }
            }

            on_select.emit(new_color.clone());
            model.set(next_model);
            editing_color.set(None);
            error_message.set(None);
            context_menu.set(None);
            modal_open.set(false);
        })
    };

    let on_custom_context_menu = {
        let context_menu = context_menu.clone();
        Callback::from(move |(name, e): (String, MouseEvent)| {
            e.prevent_default();
            context_menu.set(Some((name, e.client_x(), e.client_y())));
        })
    };

    let on_delete_custom = {
        let model = model.clone();
        let saved_custom_colors = saved_custom_colors.clone();
        let on_select = props.on_select.clone();
        let selected_name = props.selected_name.clone();
        let context_menu = context_menu.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let Some((name, _, _)) = (*context_menu).clone() else {
                return;
            };
            let mut next_model = (*model).clone();
            if !next_model.remove_custom_color(&name) {
                return;
            }

            if selected_name == name {
                if let Some(fallback) = next_model.default_colors().first().cloned() {
                    on_select.emit(fallback);
                }
            }

            let mut next_saved = (*saved_custom_colors).clone();
            let saved_len = next_saved.len();
            next_saved.retain(|color| color.name != name);
            if next_saved.len() != saved_len {
                persist_saved_custom_colors(&next_saved);
                saved_custom_colors.set(next_saved);
            }

            model.set(next_model);
            context_menu.set(None);
        })
    };

    let on_edit_custom = {
        let model = model.clone();
        let saved_custom_colors = saved_custom_colors.clone();
        let draft = draft.clone();
        let editing_color = editing_color.clone();
        let error_message = error_message.clone();
        let modal_open = modal_open.clone();
        let context_menu = context_menu.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let Some((name, _, _)) = (*context_menu).clone() else {
                return;
            };
            let Some(color) = model
                .custom_colors()
                .iter()
                .find(|entry| entry.name == name)
                .cloned()
            else {
                context_menu.set(None);
                return;
            };

            let is_saved = saved_custom_colors.iter().any(|entry| entry.name == name);
            draft.set(CustomColorDraft {
                name: color.name.clone(),
                color: color.color,
                shade: color.shade,
                border: color.border,
                text: color.text,
                save_for_later: is_saved,
            });
            editing_color.set(Some(CustomColorEditState {
                original_name: name,
            }));
            error_message.set(None);
            context_menu.set(None);
            modal_open.set(true);
        })
    };

    let keep_menu_open = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
    });

    let is_editing = editing_color.is_some();
    html! {
        <EditorGroup
            title="Colors"
            class={style::BRICK_SETTINGS_EDITOR_GROUP_CLASS}
            content_class={style::BRICK_SETTINGS_EDITOR_GROUP_CONTENT}
        >
            <div class={style::COLOR_VIEW_SHELL} onclick={close_context_menu}>
                <div class={style::COLOR_VIEW_SCROLL_WRAP}>
                    <ColorView
                        colors={colors}
                        custom_color_names={custom_color_names}
                        selected_name={props.selected_name.clone()}
                        on_select={props.on_select.clone()}
                        on_add_custom={open_modal}
                        on_custom_context_menu={on_custom_context_menu}
                    />
                </div>
                if let Some((_, x, y)) = &*context_menu {
                    <ContextMenu x={*x} y={*y} onmousedown={keep_menu_open}>
                        <ContextMenuItem onmousedown={on_edit_custom}>
                            {"Edit"}
                        </ContextMenuItem>
                        <ContextMenuItem onmousedown={on_delete_custom}>
                            {"Delete"}
                        </ContextMenuItem>
                    </ContextMenu>
                }
            </div>
            if *modal_open {
                <CustomColorModal
                    draft={(*draft).clone()}
                    {is_editing}
                    error_message={(*error_message).clone()}
                    on_change={on_draft_change}
                    on_save={on_save}
                    on_close={close_modal.clone()}
                />
            }
        </EditorGroup>
    }
}
