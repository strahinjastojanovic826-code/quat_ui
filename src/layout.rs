use crate::quat::Quat;

/// Layout behavior flags encoded in quaternary states:
/// - 00: Collapsed (hidden, takes zero space)
/// - 01: Relative (standard document flow)
/// - 10: Absolute (floating, ignores parent flow)
/// - 11: Expanded (fills all available space)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutMode {
    pub horizontal: Quat,
    pub vertical: Quat,
}

impl Default for LayoutMode {
    fn default() -> Self {
        Self {
            horizontal: Quat::V01, // Standard Relative width
            vertical: Quat::V01,   // Standard Relative height
        }
    }
}

impl LayoutMode {
    pub fn new(horizontal: Quat, vertical: Quat) -> Self {
        Self { horizontal, vertical }
    }

    /// Checks if the element is completely hidden from layout calculations.
    pub fn is_collapsed(&self) -> bool {
        self.horizontal == Quat::V00 && self.vertical == Quat::V00
    }

    /// Checks if the element expands to fill space.
    pub fn is_expanded(&self) -> bool {
        self.horizontal == Quat::V11 || self.vertical == Quat::V11
    }
}