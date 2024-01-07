#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(dead_code)]
pub enum Direction {
    N, E, S, W
}

impl Direction {
    pub fn dx(&self) -> isize {
        match self {
            Self::E => 1,
            Self::W => -1,
            _ => 0
        }
    }

    pub fn dy(&self) -> isize {
        match self {
            Self::S => 1,
            Self::N => -1,
            _ => 0
        }
    }
}
