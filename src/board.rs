use crate::{bitboards::Bitboards, Color};

pub struct Board {
    pub bitboards: Bitboards,
    pub to_move: Color,
}
