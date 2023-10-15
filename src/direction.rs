use crate::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    UpLeft = 0,
    UpRight = 1,
    DownRight = 2,
    DownLeft = 3,
}

const fn forward_right(color: Color) -> Direction {
    if color as u32 == Color::White as u32 {
        Direction::UpRight
    } else {
        Direction::DownRight
    }
}

const fn forward_left(color: Color) -> Direction {
    if color as u32 == Color::White as u32 {
        Direction::UpLeft
    } else {
        Direction::DownLeft
    }
}

const fn back_right(color: Color) -> Direction {
    if color as u32 == Color::White as u32 {
        Direction::DownRight
    } else {
        Direction::UpRight
    }
}

const fn back_left(color: Color) -> Direction {
    if color as u32 == Color::White as u32 {
        Direction::DownLeft
    } else {
        Direction::UpLeft
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::UpLeft
    }
}
