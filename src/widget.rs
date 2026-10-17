use crate::state::UiState;

/// Core trait representing any UI component in the quaternary library.
pub trait Widget {
    /// Renders the widget or logs its state representation.
    fn render(&self);

    /// Returns the current quaternary UI state.
    fn state(&self) -> UiState;

    /// Updates the widget state.
    fn set_state(&mut self, state: UiState);
}

/// A basic concrete widget implementation.
pub struct BasicWidget {
    pub id: String,
    pub state: UiState,
}

impl BasicWidget {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            state: UiState::default(),
        }
    }
}

impl Widget for BasicWidget {
    fn render(&self) {
        println!("[QuatUI] Rendering widget '{}' with state quat: {:?}", self.id, self.state.quat);
    }

    fn state(&self) -> UiState {
        self.state
    }

    fn set_state(&mut self, state: UiState) {
        self.state = state;
    }
}