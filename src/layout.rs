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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
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
    pub fn resolve_layout(&self, parent_width: f32, parent_height: f32) -> LayoutRect {
        // 00: Collapsed -> Zero space
        if self.is_collapsed() {
            return LayoutRect { x: 0.0, y: 0.0, width: 0.0, height: 0.0 };
        }

        // 11: Expanded -> Fills all available space
        let width = if self.horizontal.to_bits() == 0b11 { parent_width } else { parent_width * 0.5 };
        let height = if self.vertical.to_bits() == 0b11 { parent_height } else { parent_height * 0.5 };

        LayoutRect {
            x: 0.0,
            y: 0.0,
            width,
            height,
        }
    }

}