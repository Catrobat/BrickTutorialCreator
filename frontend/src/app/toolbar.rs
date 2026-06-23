#[cfg(target_arch = "wasm32")]
use super::{
    HELP_CONTACT_URL, HELP_DOCS_URL, ICON_ABOUT, ICON_ADD_BRICK, ICON_ADD_CUSTOM,
    ICON_BRICK_CATALOG, ICON_CHEVRON_RIGHT, ICON_DELETE, ICON_DOWNLOAD, ICON_EDIT,
    ICON_EDIT_SQUARE, ICON_EXPORT_ALL_BRICKS, ICON_EXPORT_NINEPATCH, ICON_HELP, ICON_LOADING,
    ICON_MENU, ICON_MOON, ICON_MOVE, ICON_PREVIEW, ICON_REDO, ICON_RESET, ICON_SUN, ICON_UNDO,
    ICON_UPLOAD,
};
#[cfg(target_arch = "wasm32")]
use crate::app as style;
#[cfg(target_arch = "wasm32")]
use crate::components::icon_button::IconButton;
#[cfg(target_arch = "wasm32")]
use crate::components::modal::Modal;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
fn themed_menu_icon(svg: &str) -> AttrValue {
    AttrValue::from(svg.replace("fill=\"#1f1f1f\"", "fill=\"currentColor\""))
}

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct AppToolbarProps {
    pub menu_open: bool,
    pub help_submenu_open: bool,
    pub light: bool,
    pub can_undo: bool,
    pub can_redo: bool,
    pub can_reset: bool,
    pub all_bricks_rendering: bool,
    pub ninepatch_rendering: bool,
    pub on_menu: Callback<MouseEvent>,
    pub on_menu_mouse_leave: Callback<MouseEvent>,
    pub on_undo: Callback<MouseEvent>,
    pub on_redo: Callback<MouseEvent>,
    pub on_reset: Callback<MouseEvent>,
    pub on_help: Callback<MouseEvent>,
    pub on_help_link_click: Callback<MouseEvent>,
    pub on_open_explanation: Callback<MouseEvent>,
    pub on_about: Callback<MouseEvent>,
    pub on_toggle_theme: Callback<MouseEvent>,
    pub on_import: Callback<MouseEvent>,
    pub on_enter_export_mode: Callback<MouseEvent>,
    pub on_export_all_bricks_zip: Callback<MouseEvent>,
    pub on_export_ninepatch_zip: Callback<MouseEvent>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(AppToolbar)]
pub fn app_toolbar(props: &AppToolbarProps) -> Html {
    html! {
        <div class={style::APP_TOOLBAR}>
            <div class={style::APP_TOOLBAR_GROUP}>
                <div class={style::TOOLBAR_MENU}>
                    <IconButton
                        icon={Html::from_html_unchecked(AttrValue::from(ICON_MENU))}
                        title="Menu"
                        label="Menu"
                        onclick={props.on_menu.clone()}
                    />
                    if props.menu_open {
                        <div class={style::TOOLBAR_MENU_DROPDOWN} onmouseleave={props.on_menu_mouse_leave.clone()}>
                            <button
                                class={style::TOOLBAR_MENU_ITEM}
                                type="button"
                                onclick={props.on_undo.clone()}
                                disabled={!props.can_undo}
                            >
                                <span class={style::TOOLBAR_MENU_ICON} aria-hidden="true">
                                    {Html::from_html_unchecked(themed_menu_icon(ICON_UNDO))}
                                </span>
                                <span>{"Undo"}</span>
                            </button>
                            <button
                                class={style::TOOLBAR_MENU_ITEM}
                                type="button"
                                onclick={props.on_redo.clone()}
                                disabled={!props.can_redo}
                            >
                                <span class={style::TOOLBAR_MENU_ICON} aria-hidden="true">
                                    {Html::from_html_unchecked(themed_menu_icon(ICON_REDO))}
                                </span>
                                <span>{"Redo"}</span>
                            </button>
                            <button
                                class={style::TOOLBAR_MENU_ITEM}
                                type="button"
                                onclick={props.on_reset.clone()}
                                disabled={!props.can_reset}
                            >
                                <span class={style::TOOLBAR_MENU_ICON} aria-hidden="true">
                                    {Html::from_html_unchecked(AttrValue::from(ICON_RESET))}
                                </span>
                                <span>{"Reset to default"}</span>
                            </button>
                            <div class="toolbar-menu__submenu-wrap">
                                <button
                                    class={classes!("toolbar-menu__item", "toolbar-menu__item--submenu-toggle", props.help_submenu_open.then_some("toolbar-menu__item--active"))}
                                    type="button"
                                    onclick={props.on_help.clone()}
                                >
                                    <span class="toolbar-menu__icon" aria-hidden="true">
                                        {Html::from_html_unchecked(AttrValue::from(ICON_HELP))}
                                    </span>
                                    <span>{"Help"}</span>
                                    <span class="toolbar-menu__caret" aria-hidden="true">
                                        {Html::from_html_unchecked(AttrValue::from(ICON_CHEVRON_RIGHT))}
                                    </span>
                                </button>
                                if props.help_submenu_open {
                                    <div class="toolbar-submenu">
                                        <button
                                            class="toolbar-submenu__item"
                                            type="button"
                                            onclick={props.on_open_explanation.clone()}
                                        >
                                            {"Icon Explanation"}
                                        </button>
                                        <a
                                            class="toolbar-submenu__item"
                                            href={HELP_CONTACT_URL}
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            onclick={props.on_help_link_click.clone()}
                                        >
                                            {"Contact information"}
                                        </a>
                                        <a
                                            class="toolbar-submenu__item"
                                            href={HELP_DOCS_URL}
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            onclick={props.on_help_link_click.clone()}
                                        >
                                            {"Brick Documentation"}
                                        </a>
                                    </div>
                                }
                            </div>
                            <button
                                class={style::TOOLBAR_MENU_ITEM}
                                type="button"
                                onclick={props.on_about.clone()}
                            >
                                <span class={style::TOOLBAR_MENU_ICON} aria-hidden="true">
                                    {Html::from_html_unchecked(themed_menu_icon(ICON_ABOUT))}
                                </span>
                                <span>{"About"}</span>
                            </button>
                        </div>
                    }
                </div>
                <IconButton
                    icon={Html::from_html_unchecked(AttrValue::from(if props.light { ICON_MOON } else { ICON_SUN }))}
                    title={if props.light { "Switch to dark mode" } else { "Switch to light mode" }}
                    label="Theme"
                    onclick={props.on_toggle_theme.clone()}
                />
            </div>
            <div class={style::APP_TOOLBAR_GROUP}>
                <IconButton
                    icon={Html::from_html_unchecked(AttrValue::from(ICON_UPLOAD))}
                    title="Import JSON"
                    label="Import"
                    onclick={props.on_import.clone()}
                />
                <IconButton
                    icon={Html::from_html_unchecked(AttrValue::from(ICON_DOWNLOAD))}
                    title="Export"
                    label="Export"
                    onclick={props.on_enter_export_mode.clone()}
                />
                <IconButton
                    icon={Html::from_html_unchecked(AttrValue::from(if props.all_bricks_rendering { ICON_LOADING } else { ICON_EXPORT_ALL_BRICKS }))}
                    title={if props.all_bricks_rendering { "Rendering All Bricks ZIP..." } else { "Render All Bricks ZIP" }}
                    label="All ZIP"
                    onclick={props.on_export_all_bricks_zip.clone()}
                    disabled={props.all_bricks_rendering}
                />
                <IconButton
                    icon={Html::from_html_unchecked(AttrValue::from(if props.ninepatch_rendering { ICON_LOADING } else { ICON_EXPORT_NINEPATCH }))}
                    title={if props.ninepatch_rendering { "Rendering 9-patch ZIP..." } else { "Render 9-patch ZIP" }}
                    label="9-patch ZIP"
                    onclick={props.on_export_ninepatch_zip.clone()}
                    disabled={props.ninepatch_rendering}
                />
            </div>
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct IconExplanationModalProps {
    pub on_close: Callback<MouseEvent>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(IconExplanationModal)]
pub fn icon_explanation_modal(props: &IconExplanationModalProps) -> Html {
    html! {
        <Modal
            title="Icon Explanation"
            hint="What each toolbar icon does"
            class={classes!("explanation-modal")}
            on_close={props.on_close.clone()}
        >
            <div class="explanation-modal__list">
                <ExplanationRow icon={Html::from_html_unchecked(AttrValue::from(ICON_MENU))} text="Open the main menu, where you can access undo, redo, reset to default, Help, and About." />
                <ExplanationRow icon={Html::from_html_unchecked(AttrValue::from(ICON_SUN))} text="Switch to light mode." />
                <ExplanationRow icon={Html::from_html_unchecked(AttrValue::from(ICON_MOON))} text="Switch to dark mode." />
                <ExplanationRow icon={Html::from_html_unchecked(themed_menu_icon(ICON_ADD_BRICK))} text="Add the current brick from the brick editor to the tutorial editor." />
                <ExplanationRow icon={Html::from_html_unchecked(themed_menu_icon(ICON_DELETE))} text="Delete the selected brick from the tutorial editor." />
                <ExplanationRow icon={Html::from_html_unchecked(themed_menu_icon(ICON_EDIT))} text="Apply the current changes from the brick editor to the selected brick in the tutorial editor." />
                <ExplanationRow icon={Html::from_html_unchecked(AttrValue::from(ICON_PREVIEW))} text="Show a preview of the rendered bricks in the tutorial editor." />
                <ExplanationRow icon={Html::from_html_unchecked(themed_menu_icon(ICON_EDIT_SQUARE))} text="Return from preview mode to the tutorial editor." />
                <ExplanationRow icon={Html::from_html_unchecked(AttrValue::from(ICON_BRICK_CATALOG))} text="Open the brick catalog. Double-click a prebuilt brick to add it to the tutorial editor." />
                <ExplanationRow icon={Html::from_html_unchecked(AttrValue::from(ICON_ADD_CUSTOM))} text="Add a custom color. Right-click a custom color to edit or delete it. You can also tick the option to store the color on the device and reuse it later." />
                <ExplanationRow icon={Html::from_html_unchecked(AttrValue::from(ICON_MOVE))} text="Move the selected brick up or down in the tutorial editor." />
                <ExplanationRow icon={Html::from_html_unchecked(AttrValue::from(ICON_UPLOAD))} text="Import a JSON file containing either a single brick or a full tutorial." />
                <ExplanationRow icon={Html::from_html_unchecked(AttrValue::from(ICON_DOWNLOAD))} text="Choose which bricks to export and select the output format. Use Select All, Clear Selection, and Done to finish. The checkboxes to the left of the tutorial editor show which bricks are selected." />
                <ExplanationRow icon={Html::from_html_unchecked(themed_menu_icon(ICON_EXPORT_ALL_BRICKS))} text="Download a ZIP file containing all bricks from the catalog." />
                <ExplanationRow icon={Html::from_html_unchecked(AttrValue::from(ICON_EXPORT_NINEPATCH))} text="Download the 9-patch ZIP file." />
            </div>
        </Modal>
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
struct ExplanationRowProps {
    icon: Html,
    text: AttrValue,
}

#[cfg(target_arch = "wasm32")]
#[function_component(ExplanationRow)]
fn explanation_row(props: &ExplanationRowProps) -> Html {
    html! {
        <div class="explanation-modal__row">
            <div class="explanation-modal__icon">
                {props.icon.clone()}
            </div>
            <div class="explanation-modal__text">
                {props.text.clone()}
            </div>
        </div>
    }
}
