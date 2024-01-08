pub use serde::*;
pub use postcard::{from_bytes, to_allocvec};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Maze {
    pub size_x: usize,
    pub size_y: usize,
    pub goal_x: usize,
    pub goal_y: usize,
    pub grid: Vec<Vec<bool>>,
}

static mut RAND_SEED: u64 = 0;

pub fn rand() -> u64 {
    unsafe {
        let mut x = RAND_SEED;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        RAND_SEED = x;
        x
    }
}

/// inclusive
pub fn rand_in(min: u64, max: u64) -> u64 {
    (rand() - min) % (max - min + 1) + min
}

pub fn srand(seed: u64) {
    unsafe {
        RAND_SEED = seed;
    }
}

#[derive(Default, Clone)]
pub struct RandomDirection {
    // 0000 WSEN
    visited: u8,
}

impl Iterator for RandomDirection {
    type Item = Direction;

    fn next(&mut self) -> Option<Direction> {
        if self.visited == 0b0000_1111 {
            return None;
        }

        let mut options = Vec::with_capacity(4);
        for bit in 0..4_u8 {
            if (self.visited >> bit) & 1 == 0 {
                options.push(unsafe {
                    core::mem::transmute(bit)
                });
            }
        }

        let idx = rand_in(0, options.len() as u64 - 1);
        let opt = options[idx as usize];
        self.visited |= 1 << (opt as u8);

        Some(opt)
    }
}

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
