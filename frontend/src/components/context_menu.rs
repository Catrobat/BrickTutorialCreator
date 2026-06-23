#[cfg(target_arch = "wasm32")]
use crate::components as style;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct ContextMenuProps {
    pub x: i32,
    pub y: i32,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onmousedown: Callback<MouseEvent>,
    #[prop_or_default]
    pub children: Children,
}

#[cfg(target_arch = "wasm32")]
#[function_component(ContextMenu)]
pub fn context_menu(props: &ContextMenuProps) -> Html {
    html! {
        <div
            class={classes!(style::CONTEXT_MENU, props.class.clone())}
            style={format!("left:{}px; top:{}px;", props.x, props.y)}
            onmousedown={props.onmousedown.clone()}
        >
            { for props.children.iter() }
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct ContextMenuItemProps {
    #[prop_or_default]
    pub onmousedown: Callback<MouseEvent>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: Children,
}

#[cfg(target_arch = "wasm32")]
#[function_component(ContextMenuItem)]
pub fn context_menu_item(props: &ContextMenuItemProps) -> Html {
    html! {
        <button
            class={style::CONTEXT_MENU_ITEM}
            type="button"
            onmousedown={props.onmousedown.clone()}
            disabled={props.disabled}
        >
            { for props.children.iter() }
        </button>
    }
}
