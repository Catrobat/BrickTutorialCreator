#[cfg(target_arch = "wasm32")]
use crate::app::views as style;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::components::brick_view::BrickView;
#[cfg(target_arch = "wasm32")]
use crate::components::card::Card;
#[cfg(target_arch = "wasm32")]
use crate::components::editor_group::EditorGroup;
#[cfg(target_arch = "wasm32")]
use crate::components::modal::Modal;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::brick::BrickState;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::catalog;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct BrickCatalogModalProps {
    pub on_close: Callback<MouseEvent>,
    pub on_add_brick: Callback<BrickState>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(BrickCatalogModal)]
pub fn brick_catalog_modal(props: &BrickCatalogModalProps) -> Html {
    let groups = catalog::catalog_groups();

    html! {
        <Modal
            title="Brick Catalog"
            hint="Double-click to add"
            on_close={props.on_close.clone()}
        >
            <div class={style::BRICK_CATALOG_GROUPS}>
                { for groups.into_iter().map(|group| {
                    html! {
                        <div class={style::BRICK_CATALOG_GROUP_WRAP} key={group.name.clone()}>
                            <EditorGroup
                                title={group.name}
                                class={style::BRICK_CATALOG_EDITOR_GROUP_CLASS}
                                content_class={style::BRICK_CATALOG_EDITOR_GROUP_CONTENT}
                            >
                                <div class={style::BRICK_CATALOG_GRID}>
                                    { for group.entries.into_iter().map(|entry| {
                                        let on_add_brick = props.on_add_brick.clone();
                                        let on_dblclick = if let Some(brick) = entry.brick.clone() {
                                            Callback::from(move |_| on_add_brick.emit(brick.clone()))
                                        } else {
                                            Callback::default()
                                        };

                                        html! {
                                            <Card
                                                key={entry.path}
                                                title={entry.label}
                                                class={style::BRICK_CATALOG_CARD}
                                                selectable={true}
                                                ondblclick={on_dblclick}
                                            >
                                                <div class={style::BRICK_CATALOG_PREVIEW_FRAME} title={entry.path}>
                                                    if let Some(brick) = entry.brick {
                                                        <BrickView brick={brick} class={style::BRICK_CATALOG_PREVIEW_IMAGE} />
                                                    } else {
                                                        <div class={style::BRICK_CATALOG_INVALID}>{ "Invalid brick" }</div>
                                                    }
                                                </div>
                                            </Card>
                                        }
                                    }) }
                                </div>
                            </EditorGroup>
                        </div>
                    }
                }) }
            </div>
        </Modal>
    }
}
