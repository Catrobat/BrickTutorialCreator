use std::ops::Deref;
use std::rc::Rc;

use shared::tutorial::{
    TutorialState, tutorial_from_brick_models, tutorial_png_bytes as shared_tutorial_png_bytes,
};
use yew::Reducible;

use crate::interfaces::brick::BrickState;

#[derive(Clone, Default, PartialEq)]
pub struct TutorialViewState {
    state: TutorialState,
}

pub enum TutorialAction {
    AddBrick(BrickState),
    InsertAfterSelected(Vec<BrickState>),
    RemoveSelected,
    ApplyChanges(BrickState),
    Select(usize),
    Deselect,
    MoveEntry(usize, usize),
    LoadJson(String),
    Restore(TutorialViewState),
}

pub fn tutorial_from_states(states: &[BrickState], name: &str) -> shared::tutorial::Tutorial {
    let models = states
        .iter()
        .map(|state| state.as_model().clone())
        .collect::<Vec<_>>();
    tutorial_from_brick_models(&models, name)
}

pub fn tutorial_png_bytes(states: &[BrickState], target_width: u32) -> Result<Vec<u8>, String> {
    let models = states
        .iter()
        .map(|state| state.as_model().clone())
        .collect::<Vec<_>>();
    shared_tutorial_png_bytes(&models, target_width)
}

impl Reducible for TutorialViewState {
    type Action = TutorialAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            TutorialAction::AddBrick(brick) => {
                let mut state = self.state.clone();
                state.add_brick(brick.as_model());
                Rc::new(Self::from_state(state))
            }
            TutorialAction::InsertAfterSelected(bricks) => {
                let mut state = self.state.clone();
                let models = bricks
                    .iter()
                    .map(|brick| brick.as_model().clone())
                    .collect::<Vec<_>>();
                state.insert_after_selected(&models);
                Rc::new(Self::from_state(state))
            }
            TutorialAction::RemoveSelected => {
                let mut state = self.state.clone();
                state.remove_selected();
                Rc::new(Self::from_state(state))
            }
            TutorialAction::ApplyChanges(brick) => {
                let mut state = self.state.clone();
                let _ = state.apply_changes(brick.as_model());
                Rc::new(Self::from_state(state))
            }
            TutorialAction::Select(index) => {
                let mut state = self.state.clone();
                state.select(index);
                Rc::new(Self::from_state(state))
            }
            TutorialAction::Deselect => {
                let mut state = self.state.clone();
                state.deselect();
                Rc::new(Self::from_state(state))
            }
            TutorialAction::MoveEntry(from, to) => {
                let mut state = self.state.clone();
                let _ = state.move_entry(from, to);
                Rc::new(Self::from_state(state))
            }
            TutorialAction::LoadJson(text) => {
                let mut state = self.state.clone();
                match state.load_json(&text) {
                    Ok(()) => Rc::new(Self::from_state(state)),
                    Err(e) => {
                        web_sys::console::error_1(&format!("Import error: {e}").into());
                        self
                    }
                }
            }
            TutorialAction::Restore(state) => Rc::new(state),
        }
    }
}

impl Deref for TutorialViewState {
    type Target = TutorialState;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl TutorialViewState {
    pub fn from_state(state: TutorialState) -> Self {
        Self { state }
    }

    pub fn get_brick_state_list(&self) -> Vec<BrickState> {
        self.state
            .get_brick_models()
            .into_iter()
            .map(BrickState::from_model)
            .collect()
    }

    pub fn to_json(&self) -> String {
        self.state.to_json()
    }

    pub fn get_png_bytes(&self, target_width: u32) -> Result<Vec<u8>, String> {
        self.state.get_png_bytes(target_width)
    }

    pub fn get_png(&self, target_width: u32) -> Result<String, String> {
        self.state.get_png(target_width)
    }
}
