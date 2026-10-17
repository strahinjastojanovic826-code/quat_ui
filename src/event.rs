use crate::quat::Quat;

/// Represents high-level user interactions or system triggers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiEvent {
    Disable,
    Enable,
    FocusGained,
    FocusLost,
    Press,
    Release,
}

impl UiEvent {
    /// Automatically maps a UI event to its corresponding quaternary state.
    pub fn target_quat(&self) -> Quat {
        match self {
            UiEvent::Disable => Quat::V00,         // 00: Disabled
            UiEvent::Enable | UiEvent::FocusLost => Quat::V01, // 01: Normal / Idle
            UiEvent::FocusGained => Quat::V10,    // 10: Hovered / Focused
            UiEvent::Press | UiEvent::Release => Quat::V11,    // 11: Active / Pressed
        }
    }
}