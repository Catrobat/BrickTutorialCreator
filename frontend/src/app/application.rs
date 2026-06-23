#[cfg(target_arch = "wasm32")]
use super::ABOUT_MESSAGE;
#[cfg(target_arch = "wasm32")]
use crate::app as style;
#[cfg(target_arch = "wasm32")]
use crate::app::browser::{cached_download_callback, upload_json};
#[cfg(target_arch = "wasm32")]
use crate::app::editors::brick_editor::BrickEditor;
#[cfg(target_arch = "wasm32")]
use crate::app::editors::tutorial_editor::TutorialEditor;
#[cfg(target_arch = "wasm32")]
use crate::app::export_selection;
#[cfg(target_arch = "wasm32")]
use crate::app::history::{AppSnapshot, next_history, pending_restore_count};
#[cfg(target_arch = "wasm32")]
use crate::app::import_export::{
    ALL_BRICKS_TILE_WIDTH, export_selected_json, export_selected_png, export_selected_svg,
    import_json_text,
};
#[cfg(target_arch = "wasm32")]
use crate::app::theme::{apply_theme, load_saved_theme};
#[cfg(target_arch = "wasm32")]
use crate::app::toolbar::{AppToolbar, IconExplanationModal};
#[cfg(target_arch = "wasm32")]
use crate::components::sidebar::Sidebar;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::brick::BrickState;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::catalog;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::ninepatch;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::tutorial::{TutorialAction, TutorialViewState};
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
fn close_menu(menu_open: &UseStateHandle<bool>, help_submenu_open: &UseStateHandle<bool>) {
    menu_open.set(false);
    help_submenu_open.set(false);
}

#[cfg(target_arch = "wasm32")]
#[function_component(App)]
fn app() -> Html {
    let brick = use_reducer(BrickState::default);
    let tutorial = use_reducer(TutorialViewState::default);

    let export_mode = use_state(|| false);
    let all_bricks_url = use_state(|| Option::<String>::None);
    let all_bricks_rendering = use_state(|| false);
    let ninepatch_url = use_state(|| Option::<String>::None);
    let ninepatch_rendering = use_state(|| false);
    let menu_open = use_state(|| false);
    let help_submenu_open = use_state(|| false);
    let explanation_modal_open = use_state(|| false);
    let history = use_state(|| vec![AppSnapshot::default()]);
    let history_index = use_state(|| 0usize);
    let restoring_history = use_mut_ref(|| 0usize);
    let light = use_state(|| {
        let saved = load_saved_theme();
        apply_theme(saved);
        saved
    });

    {
        let history = history.clone();
        let history_index = history_index.clone();
        let restoring_history = restoring_history.clone();
        let snapshot = AppSnapshot {
            brick: (*brick).clone(),
            tutorial: (*tutorial).clone(),
        };
        use_effect_with(snapshot, move |snapshot| {
            let pending_restores = *restoring_history.borrow();
            if pending_restores > 0 {
                *restoring_history.borrow_mut() = pending_restores - 1;
            } else {
                if let Some((next, next_index)) = next_history(&*history, *history_index, snapshot)
                {
                    history.set(next);
                    history_index.set(next_index);
                }
            }
            || ()
        });
    }

    let tutorial_len = (*tutorial).tutorial.content.len();
    let export_selection = use_state(|| vec![false; tutorial_len]);
    {
        let export_selection = export_selection.clone();
        use_effect_with(tutorial_len, move |len| {
            export_selection.set(export_selection::resized(*len));
            || ()
        });
    }

    let on_toggle_export = {
        let export_selection = export_selection.clone();
        Callback::from(move |index: usize| {
            export_selection.set(export_selection::toggled(&*export_selection, index));
        })
    };

    let on_export_select_all = {
        let export_selection = export_selection.clone();
        Callback::from(move |_: MouseEvent| {
            export_selection.set(export_selection::all_selected((*export_selection).len()));
        })
    };

    let on_export_clear = {
        let export_selection = export_selection.clone();
        Callback::from(move |_: MouseEvent| {
            export_selection.set(export_selection::cleared((*export_selection).len()));
        })
    };

    let brick_dispatcher = brick.dispatcher();
    let tutorial_dispatcher = tutorial.dispatcher();

    let import_json = {
        let tutorial_dispatcher = tutorial_dispatcher.clone();
        Callback::from(move |_: ()| {
            let tutorial_dispatcher = tutorial_dispatcher.clone();
            upload_json(Callback::from(move |text: String| {
                import_json_text(text, &tutorial_dispatcher);
            }));
        })
    };

    let on_export_selected_json = {
        let tutorial = tutorial.clone();
        let export_selection = export_selection.clone();
        let export_mode = export_mode.clone();
        Callback::from(move |_: MouseEvent| {
            let selection = (*export_selection).clone();
            match export_selected_json(&*tutorial, &selection) {
                Ok(()) => export_mode.set(false),
                Err(error) => web_sys::console::error_1(&error.into()),
            }
        })
    };

    let on_export_selected_png = {
        let tutorial = tutorial.clone();
        let export_selection = export_selection.clone();
        let export_mode = export_mode.clone();
        Callback::from(move |_: MouseEvent| {
            let selection = (*export_selection).clone();
            match export_selected_png(&*tutorial, &selection) {
                Ok(()) => export_mode.set(false),
                Err(error) => web_sys::console::error_1(&error.into()),
            }
        })
    };

    let on_export_selected_svg = {
        let tutorial = tutorial.clone();
        let export_selection = export_selection.clone();
        let export_mode = export_mode.clone();
        Callback::from(move |_: MouseEvent| {
            let selection = (*export_selection).clone();
            match export_selected_svg(&*tutorial, &selection) {
                Ok(()) => export_mode.set(false),
                Err(error) => web_sys::console::error_1(&error.into()),
            }
        })
    };

    let export_all_bricks_zip = cached_download_callback(
        all_bricks_url.clone(),
        all_bricks_rendering.clone(),
        "all_bricks.zip",
        "All bricks render error:",
        Rc::new(|| {
            catalog::render_all_bricks_zip_bytes(ALL_BRICKS_TILE_WIDTH).map_err(|e| e.to_string())
        }),
    );

    let export_ninepatch_zip = cached_download_callback(
        ninepatch_url.clone(),
        ninepatch_rendering.clone(),
        "ninepatch_bricks.zip",
        "Ninepatch render error:",
        Rc::new(|| ninepatch::render_ninepatch_zip_bytes().map_err(|e| e.to_string())),
    );

    let on_enter_export_mode = {
        let export_mode = export_mode.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            export_mode.set(true);
        })
    };

    let on_exit_export_mode = {
        let export_mode = export_mode.clone();
        Callback::from(move |_: MouseEvent| export_mode.set(false))
    };

    let on_import = {
        let import_json = import_json.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            import_json.emit(());
        })
    };

    let toggle_theme = {
        let light = light.clone();
        Callback::from(move |_: MouseEvent| {
            let new_val = !*light;
            apply_theme(new_val);
            light.set(new_val);
        })
    };

    let on_menu = {
        let menu_open = menu_open.clone();
        let help_submenu_open = help_submenu_open.clone();
        Callback::from(move |_: MouseEvent| {
            let next_open = !*menu_open;
            menu_open.set(next_open);
            if !next_open {
                help_submenu_open.set(false);
            }
        })
    };

    let on_menu_mouse_leave = {
        let menu_open = menu_open.clone();
        let help_submenu_open = help_submenu_open.clone();
        Callback::from(move |_: MouseEvent| {
            close_menu(&menu_open, &help_submenu_open);
        })
    };

    let on_undo = {
        let history = history.clone();
        let history_index = history_index.clone();
        let restoring_history = restoring_history.clone();
        let brick = brick.clone();
        let tutorial = tutorial.clone();
        let brick_dispatcher = brick_dispatcher.clone();
        let tutorial_dispatcher = tutorial_dispatcher.clone();
        Callback::from(move |_: MouseEvent| {
            let current_index = *history_index;
            if current_index == 0 {
                return;
            }
            let target_index = current_index - 1;
            if let Some(snapshot) = (*history).get(target_index).cloned() {
                let pending_restores = pending_restore_count(&*brick, &*tutorial, &snapshot);
                *restoring_history.borrow_mut() = pending_restores;
                brick_dispatcher
                    .dispatch(crate::interfaces::brick::StateAction::Set(snapshot.brick));
                tutorial_dispatcher.dispatch(TutorialAction::Restore(snapshot.tutorial));
                history_index.set(target_index);
            }
        })
    };

    let on_redo = {
        let history = history.clone();
        let history_index = history_index.clone();
        let restoring_history = restoring_history.clone();
        let brick = brick.clone();
        let tutorial = tutorial.clone();
        let brick_dispatcher = brick_dispatcher.clone();
        let tutorial_dispatcher = tutorial_dispatcher.clone();
        Callback::from(move |_: MouseEvent| {
            let current_index = *history_index;
            let target_index = current_index + 1;
            if let Some(snapshot) = (*history).get(target_index).cloned() {
                let pending_restores = pending_restore_count(&*brick, &*tutorial, &snapshot);
                *restoring_history.borrow_mut() = pending_restores;
                brick_dispatcher
                    .dispatch(crate::interfaces::brick::StateAction::Set(snapshot.brick));
                tutorial_dispatcher.dispatch(TutorialAction::Restore(snapshot.tutorial));
                history_index.set(target_index);
            }
        })
    };

    let on_reset = {
        let menu_open = menu_open.clone();
        let help_submenu_open = help_submenu_open.clone();
        let brick_dispatcher = brick_dispatcher.clone();
        Callback::from(move |_: MouseEvent| {
            brick_dispatcher.dispatch(crate::interfaces::brick::StateAction::Reset);
            close_menu(&menu_open, &help_submenu_open);
        })
    };

    let on_help = {
        let help_submenu_open = help_submenu_open.clone();
        Callback::from(move |_: MouseEvent| {
            help_submenu_open.set(!*help_submenu_open);
        })
    };

    let on_help_link_click = {
        let menu_open = menu_open.clone();
        let help_submenu_open = help_submenu_open.clone();
        Callback::from(move |_: MouseEvent| {
            close_menu(&menu_open, &help_submenu_open);
        })
    };

    let on_open_explanation = {
        let menu_open = menu_open.clone();
        let help_submenu_open = help_submenu_open.clone();
        let explanation_modal_open = explanation_modal_open.clone();
        Callback::from(move |_: MouseEvent| {
            close_menu(&menu_open, &help_submenu_open);
            explanation_modal_open.set(true);
        })
    };

    let on_close_explanation = {
        let explanation_modal_open = explanation_modal_open.clone();
        Callback::from(move |_: MouseEvent| explanation_modal_open.set(false))
    };

    let on_about = {
        let menu_open = menu_open.clone();
        let help_submenu_open = help_submenu_open.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(window) = web_sys::window() {
                let _ = window.alert_with_message(ABOUT_MESSAGE);
            }
            close_menu(&menu_open, &help_submenu_open);
        })
    };

    let tutorial_bricks = (*tutorial).get_brick_state_list();
    let selected_count = (*export_selection)
        .iter()
        .filter(|selected| **selected)
        .count();
    let can_undo = *history_index > 0;
    let can_redo = *history_index + 1 < (*history).len();
    let can_reset = *brick != BrickState::default();
    html! {
        <div class={style::APP_ROOT}>
            <AppToolbar
                menu_open={*menu_open}
                help_submenu_open={*help_submenu_open}
                light={*light}
                {can_undo}
                {can_redo}
                {can_reset}
                all_bricks_rendering={*all_bricks_rendering}
                ninepatch_rendering={*ninepatch_rendering}
                on_menu={on_menu.clone()}
                on_menu_mouse_leave={on_menu_mouse_leave}
                on_undo={on_undo.clone()}
                on_redo={on_redo.clone()}
                on_reset={on_reset.clone()}
                on_help={on_help.clone()}
                on_help_link_click={on_help_link_click.clone()}
                on_open_explanation={on_open_explanation.clone()}
                on_about={on_about.clone()}
                on_toggle_theme={toggle_theme.clone()}
                on_import={on_import.clone()}
                on_enter_export_mode={on_enter_export_mode.clone()}
                on_export_all_bricks_zip={export_all_bricks_zip.clone()}
                on_export_ninepatch_zip={export_ninepatch_zip.clone()}
            />
            <div class={style::APP_MAIN}>
                <div class={style::APP_EDITOR_WRAP}>
                    <BrickEditor
                        brick={(*brick).clone()}
                        dispatcher={brick_dispatcher.clone()}
                        tutorial_dispatcher={tutorial_dispatcher.clone()}
                    />
                </div>
                <Sidebar>
                    <TutorialEditor
                        brick={(*brick).clone()}
                        brick_dispatcher={brick_dispatcher.clone()}
                        tutorial={(*tutorial).clone()}
                        tutorial_dispatcher={tutorial_dispatcher.clone()}
                        export_selection={(*export_selection).clone()}
                        on_toggle_export={on_toggle_export.clone()}
                        export_mode={*export_mode}
                        selected_count={selected_count}
                        total_bricks={tutorial_bricks.len()}
                        on_export_select_all={on_export_select_all.clone()}
                        on_export_clear={on_export_clear.clone()}
                        on_export_json={on_export_selected_json.clone()}
                        on_export_png={on_export_selected_png.clone()}
                        on_export_svg={on_export_selected_svg.clone()}
                        on_exit_export={on_exit_export_mode.clone()}
                    />
                </Sidebar>
            </div>
            if *explanation_modal_open {
                <IconExplanationModal on_close={on_close_explanation.clone()} />
            }
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
pub fn mount_app() {
    yew::Renderer::<App>::new().render();
}
