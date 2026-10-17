/// Represents a quaternary digit (quat) packed into 2 bits.
/// States: 00, 01, 10, 11
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Quat {
    V00 = 0b00,
    V01 = 0b01,
    V10 = 0b10,
    V11 = 0b11,
}

impl Quat {
    /// Creates a Quat from a raw 2-bit value (masked to lowest 2 bits).
    pub fn from_bits(bits: u8) -> Self {
        match bits & 0b11 {
            0b00 => Quat::V00,
            0b01 => Quat::V01,
            0b10 => Quat::V10,
            _ => Quat::V11,
        }
    }

    /// Returns the raw 2-bit representation as a u8.
    pub fn to_bits(self) -> u8 {
        self as u8
    }
}

/// A container holding up to 4 quats packed into a single u8 byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuatByte(pub u8);

impl QuatByte {
    /// Creates a new empty QuatByte.
    pub fn new() -> Self {
        Self(0)
    }

    /// Extracts a quat at the given index (0 to 3).
    pub fn get(&self, index: usize) -> Option<Quat> {
        if index >= 4 {
            return None;
        }
        let shift = (3 - index) * 2;
        let bits = (self.0 >> shift) & 0b11;
        Some(Quat::from_bits(bits))
    }

    /// Sets a quat at the given index (0 to 3).
    pub fn set(&mut self, index: usize, quat: Quat) -> Result<(), &'static str> {
        if index >= 4 {
            return Err("Index out of bounds for QuatByte (max 4 quats)");
        }
        let shift = (3 - index) * 2;
        let clear_mask = !(0b11 << shift);
        self.0 = (self.0 & clear_mask) | (quat.to_bits() << shift);
        Ok(())
    }
}

impl Default for QuatByte {
    fn default() -> Self {
        Self::new()
    }
}