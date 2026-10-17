use crate::state::UiState;
use crate::widget::Widget;

/// A container widget capable of holding child widgets (Widget Tree).
pub struct Container {
    pub id: String,
    pub state: UiState,
    children: Vec<Box<dyn Widget>>,
}

impl Container {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            state: UiState::default(),
            children: Vec::new(),
        }
    }

    /// Adds a child widget to this container.
    pub fn add_child(&mut self, widget: impl Widget + 'static) {
        self.children.push(Box::new(widget));
    }

    /// Returns the number of children in this container.
    pub fn child_count(&self) -> usize {
        self.children.len()
    }
}

impl Widget for Container {
    fn render(&self) {
        println!(
            "[QuatUI Container] Rendering container '{}' [State: {:?}] with {} children:",
            self.id, self.state.quat, self.children.len()
        );
        for (i, child) in self.children.iter().enumerate() {
            print!("    [Child {}] ", i);
            child.render();
        }
    }

    fn state(&self) -> UiState {
        self.state
    }

    fn set_state(&mut self, state: UiState) {
        self.state = state;
    }
}