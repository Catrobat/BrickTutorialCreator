#[cfg(target_arch = "wasm32")]
use crate::components as style;
#[cfg(target_arch = "wasm32")]
use crate::components::modal::Modal;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::color::CustomColorDraft;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlInputElement;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct CustomColorModalProps {
    pub draft: CustomColorDraft,
    pub is_editing: bool,
    #[prop_or_default]
    pub error_message: Option<String>,
    pub on_change: Callback<CustomColorDraft>,
    pub on_save: Callback<MouseEvent>,
    pub on_close: Callback<MouseEvent>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(CustomColorModal)]
pub fn custom_color_modal(props: &CustomColorModalProps) -> Html {
    let update_name = {
        let draft = props.draft.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                let mut next = draft.clone();
                next.name = input.value();
                on_change.emit(next);
            }
        })
    };

    let update_hex = {
        let draft = props.draft.clone();
        let on_change = props.on_change.clone();
        move |setter: fn(&mut CustomColorDraft) -> &mut String| {
            let draft = draft.clone();
            let on_change = on_change.clone();
            Callback::from(move |e: InputEvent| {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    let mut next = draft.clone();
                    *setter(&mut next) = input.value();
                    on_change.emit(next);
                }
            })
        }
    };

    let update_save_for_later = {
        let draft = props.draft.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |e: Event| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                let mut next = draft.clone();
                next.save_for_later = input.checked();
                on_change.emit(next);
            }
        })
    };

    let modal_title = if props.is_editing {
        "Edit Custom Color"
    } else {
        "Create Custom Color"
    };
    let modal_hint = if props.is_editing {
        "Update this custom color for the current session, and optionally keep the changes saved on this device."
    } else {
        "Add a reusable color scheme for this session or save it for later on this device."
    };
    let save_button_label = if props.is_editing {
        "Save changes"
    } else {
        "Save color"
    };

    html! {
        <Modal
            title={modal_title}
            hint={modal_hint}
            class={style::COLOR_MODAL_ROOT}
            body_class={style::COLOR_MODAL_BODY}
            on_close={props.on_close.clone()}
        >
            <div class={style::COLOR_MODAL_GRID}>
                <div class={style::COLOR_MODAL_FORM}>
                    <label class={style::COLOR_MODAL_FIELD}>
                        <span class={style::COLOR_MODAL_LABEL}>{"Name"}</span>
                        <input
                            class={style::COLOR_MODAL_TEXT_INPUT}
                            type="text"
                            value={props.draft.name.clone()}
                            placeholder="Salami Red"
                            oninput={update_name}
                        />
                    </label>
                    <div class={style::COLOR_MODAL_FIELD_ROW}>
                        <label class={style::COLOR_MODAL_FIELD}>
                            <span class={style::COLOR_MODAL_LABEL}>{"Main color"}</span>
                            <span class={style::COLOR_MODAL_COLOR_INPUT_WRAP}>
                                <input
                                    class={style::COLOR_MODAL_COLOR_INPUT}
                                    type="color"
                                    value={props.draft.color.clone()}
                                    oninput={update_hex(|draft| &mut draft.color)}
                                />
                                <span class={style::COLOR_MODAL_COLOR_VALUE}>{props.draft.color.clone()}</span>
                            </span>
                        </label>
                        <label class={style::COLOR_MODAL_FIELD}>
                            <span class={style::COLOR_MODAL_LABEL}>{"Shade"}</span>
                            <span class={style::COLOR_MODAL_COLOR_INPUT_WRAP}>
                                <input
                                    class={style::COLOR_MODAL_COLOR_INPUT}
                                    type="color"
                                    value={props.draft.shade.clone()}
                                    oninput={update_hex(|draft| &mut draft.shade)}
                                />
                                <span class={style::COLOR_MODAL_COLOR_VALUE}>{props.draft.shade.clone()}</span>
                            </span>
                        </label>
                    </div>
                    <div class={style::COLOR_MODAL_FIELD_ROW}>
                        <label class={style::COLOR_MODAL_FIELD}>
                            <span class={style::COLOR_MODAL_LABEL}>{"Border"}</span>
                            <span class={style::COLOR_MODAL_COLOR_INPUT_WRAP}>
                                <input
                                    class={style::COLOR_MODAL_COLOR_INPUT}
                                    type="color"
                                    value={props.draft.border.clone()}
                                    oninput={update_hex(|draft| &mut draft.border)}
                                />
                                <span class={style::COLOR_MODAL_COLOR_VALUE}>{props.draft.border.clone()}</span>
                            </span>
                        </label>
                        <label class={style::COLOR_MODAL_FIELD}>
                            <span class={style::COLOR_MODAL_LABEL}>{"Text"}</span>
                            <span class={style::COLOR_MODAL_COLOR_INPUT_WRAP}>
                                <input
                                    class={style::COLOR_MODAL_COLOR_INPUT}
                                    type="color"
                                    value={props.draft.text.clone()}
                                    oninput={update_hex(|draft| &mut draft.text)}
                                />
                                <span class={style::COLOR_MODAL_COLOR_VALUE}>{props.draft.text.clone()}</span>
                            </span>
                        </label>
                    </div>
                    <label class={style::COLOR_MODAL_CHECKBOX_ROW}>
                        <input
                            class={style::COLOR_MODAL_CHECKBOX}
                            type="checkbox"
                            checked={props.draft.save_for_later}
                            onchange={update_save_for_later}
                        />
                        <span class={style::COLOR_MODAL_CHECKBOX_TEXT}>
                            <span class={style::COLOR_MODAL_CHECKBOX_TITLE}>{"Save for later on this device"}</span>
                            <span class={style::COLOR_MODAL_CHECKBOX_HINT}>{"Stored in browser storage so it shows up again next time."}</span>
                        </span>
                    </label>
                    if let Some(message) = &props.error_message {
                        <div class={style::COLOR_MODAL_ERROR}>{message.clone()}</div>
                    }
                    <div class={style::COLOR_MODAL_ACTIONS}>
                        <button
                            class={style::COLOR_MODAL_BUTTON}
                            type="button"
                            onclick={props.on_close.clone()}
                        >
                            {"Cancel"}
                        </button>
                        <button
                            class={classes!(
                                style::COLOR_MODAL_BUTTON,
                                style::COLOR_MODAL_BUTTON_PRIMARY,
                            )}
                            type="button"
                            onclick={props.on_save.clone()}
                        >
                            {save_button_label}
                        </button>
                    </div>
                </div>
                <div class={style::COLOR_MODAL_PREVIEW}>
                    <div class={style::COLOR_MODAL_PREVIEW_TITLE}>{"Preview"}</div>
                    <div class={style::COLOR_MODAL_PREVIEW_CARD}>
                        <div class={style::COLOR_MODAL_PREVIEW_NAME}>
                            {
                                if props.draft.name.trim().is_empty() {
                                    "Custom color".to_string()
                                } else {
                                    props.draft.name.trim().to_string()
                                }
                            }
                        </div>
                        <div class={style::COLOR_MODAL_PREVIEW_SWATCH_ROW}>
                            <div
                                class={style::COLOR_MODAL_PREVIEW_SWATCH}
                                style={format!(
                                    "background:{};color:{};border:3px solid {};",
                                    props.draft.color, props.draft.text, props.draft.border
                                )}
                            >
                                {"abc"}
                            </div>
                            <div
                                class={style::COLOR_MODAL_PREVIEW_SWATCH}
                                style={format!(
                                    "background:{};color:{};border:3px solid {};",
                                    props.draft.shade, props.draft.text, props.draft.border
                                )}
                            >
                                {"abc"}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </Modal>
    }
}
