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

    /// Computes the next quaternary state from `current` given `self`.
    pub fn next_state(&self, current: Quat) -> Quat {
        // If disabled (`00`), only `Enable` can re-enable the widget (`01`).
        if current == Quat::V00 && !matches!(self, UiEvent::Enable) {
            return Quat::V00;
        }

        match self {
            UiEvent::Disable => Quat::V00,       // 00: Disabled
            UiEvent::Enable => Quat::V01,        // 01: Normal / Idle
            UiEvent::FocusGained => Quat::V10,   // 10: Hovered / Focused
            UiEvent::FocusLost => Quat::V01,     // 01: Normal / Idle
            UiEvent::Press => Quat::V11,         // 11: Active / Pressed
            UiEvent::Release => {
                if current == Quat::V11 {
                    Quat::V10
                } else {
                    current
                }
            }
        }
    }

    /// Returns true if transitioning from `current` to `next` is valid in quaternary UI logic.
    pub fn is_valid_transition(current: Quat, next: Quat) -> bool {
        if current == Quat::V00 {
            return next == Quat::V00 || next == Quat::V01;
        }
        true
    }
}