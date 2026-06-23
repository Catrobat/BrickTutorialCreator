#[cfg(target_arch = "wasm32")]
use crate::components as style;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct ExportBarProps {
    pub selected_count: usize,
    pub total_items: usize,
    pub on_select_all: Callback<MouseEvent>,
    pub on_clear: Callback<MouseEvent>,
    pub on_export_json: Callback<MouseEvent>,
    pub on_export_png: Callback<MouseEvent>,
    pub on_export_svg: Callback<MouseEvent>,
    pub on_done: Callback<MouseEvent>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(ExportBar)]
pub fn export_bar(props: &ExportBarProps) -> Html {
    html! {
        <div class={style::TUTORIAL_EDITOR_EXPORT_BAR}>
            <div class={style::TUTORIAL_EDITOR_EXPORT_STATUS}>
                {format!("Selected {} of {}", props.selected_count, props.total_items)}
            </div>
            <button class={style::TUTORIAL_EDITOR_EXPORT_BUTTON} type="button" onclick={props.on_select_all.clone()} disabled={props.total_items == 0}>{"Select all"}</button>
            <button class={style::TUTORIAL_EDITOR_EXPORT_BUTTON} type="button" onclick={props.on_clear.clone()} disabled={props.total_items == 0}>{"Clear"}</button>
            <button class={style::TUTORIAL_EDITOR_EXPORT_BUTTON} type="button" onclick={props.on_export_json.clone()} disabled={props.selected_count == 0}>{"Export JSON"}</button>
            <button class={style::TUTORIAL_EDITOR_EXPORT_BUTTON} type="button" onclick={props.on_export_png.clone()} disabled={props.selected_count == 0}>{"Export PNG"}</button>
            <button class={style::TUTORIAL_EDITOR_EXPORT_BUTTON} type="button" onclick={props.on_export_svg.clone()} disabled={props.selected_count == 0}>{"Export SVG"}</button>
            <button class={style::TUTORIAL_EDITOR_EXPORT_BUTTON} type="button" onclick={props.on_done.clone()}>{"Done"}</button>
        </div>
    }
}
