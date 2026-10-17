use crate::quat::Quat;

/// UI component states represented using quaternary logic:
/// - 00: Disabled
/// - 01: Normal / Idle
/// - 10: Hovered / Focused
/// - 11: Active / Pressed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiState {
    pub quat: Quat,
}

impl Default for UiState {
    fn default() -> Self {
        Self { quat: Quat::V01 } // Default to Normal/Idle
    }
}

impl UiState {
    pub fn new(quat: Quat) -> Self {
        Self { quat }
    }

    pub fn is_disabled(&self) -> bool {
        self.quat == Quat::V00
    }

    pub fn is_active(&self) -> bool {
        self.quat == Quat::V11
    }
}