#[derive(Copy, Clone, Debug)]
pub struct BitPosition {
    pub(crate) right_index: u32,
}

impl BitPosition {
    pub fn shift(self, x: i32, y: i32) -> Self {
        debug_assert!(
            x + (self.right_index as i32 % 8) >= 0 && x + (self.right_index as i32 % 8) < 8,
            "Attempted to shift a bit left/right outside of the board"
        );

        let right_index = self.right_index as i32 + (8 * y + x);

        debug_assert!(
            right_index >= 0 && right_index < 64,
            "Shifted bit position outside of the board"
        );

        BitPosition {
            right_index: right_index as u32,
        }
    }

    pub fn is_leftmost(self) -> bool {
        self.right_index % 8 == 0
    }

    pub fn is_rightmost(self) -> bool {
        self.right_index % 8 == 7
    }
}

impl From<u32> for BitPosition {
    fn from(right_index: u32) -> Self {
        BitPosition {
            right_index: right_index,
        }
    }
}

impl From<(u8, u8)> for BitPosition {
    fn from((rank, file): (u8, u8)) -> Self {
        BitPosition {
            right_index: (rank * 8 + file) as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
