use std::rc::Rc;

use shared::color::ColorScheme;
use shared::common::BrickRenderable;
use shared::model::BrickModel;
use shared::types::BrickType;
use yew::Reducible;

#[derive(Clone, Default, PartialEq)]
pub struct BrickState {
    model: BrickModel,
}

pub enum StateAction {
    ChangeType(BrickType),
    ChangeColor(ColorScheme),
    ChangeOffset(f32, f32),
    ChangeContent(String),
    LoadJson(String),
    Reset,
    Set(BrickState),
}

impl Reducible for BrickState {
    type Action = StateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            StateAction::ChangeType(new_type) => {
                let mut new_state = (*self).clone();
                new_state.change_type(new_type);
                Rc::new(new_state)
            }
            StateAction::ChangeColor(new_color) => {
                let mut new_state = (*self).clone();
                new_state.model.change_color(new_color);
                Rc::new(new_state)
            }
            StateAction::ChangeOffset(x, y) => {
                let mut new_state = (*self).clone();
                new_state.model.change_offset(x, y);
                Rc::new(new_state)
            }
            StateAction::ChangeContent(new_content) => {
                let mut new_state = (*self).clone();
                new_state.model.change_content(new_content);
                Rc::new(new_state)
            }
            StateAction::LoadJson(text) => match BrickState::from_json(&text) {
                Ok(brick) => Rc::new(brick),
                Err(e) => {
                    web_sys::console::error_1(&format!("Import error: {e}").into());
                    self
                }
            },
            StateAction::Reset => Rc::new(BrickState::default()),
            StateAction::Set(brick) => Rc::new(brick),
        }
    }
}

impl std::fmt::Display for BrickState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.model)
    }
}

impl BrickState {
    pub fn from_model(model: BrickModel) -> Self {
        Self { model }
    }

    pub fn as_model(&self) -> &BrickModel {
        &self.model
    }

    pub fn from_brick(brick: &dyn BrickRenderable) -> Self {
        Self {
            model: BrickModel::from_brick(brick),
        }
    }

    pub fn get_type(&self) -> BrickType {
        self.model.get_type()
    }

    pub fn change_type(&mut self, brick_type: BrickType) {
        self.model.change_type(brick_type);
    }

    pub fn as_brick(&self) -> &dyn BrickRenderable {
        self.model.as_brick()
    }

    pub fn as_mut_brick(&mut self) -> &mut dyn BrickRenderable {
        self.model.as_mut_brick()
    }

    pub fn to_box(&self) -> Box<dyn BrickRenderable> {
        self.model.to_box()
    }

    pub fn get_svg(&self) -> String {
        self.model.get_svg()
    }

    pub fn get_png(&self, target_width: u32) -> Result<Vec<u8>, String> {
        self.model.get_png(target_width)
    }

    pub fn from_json(json: &str) -> Result<Self, String> {
        BrickModel::from_json(json).map(Self::from_model)
    }
}
