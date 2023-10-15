use crate::{board::Board, Move};

const MAX_MOVES: usize = 36;
pub struct MoveGen {
    moves: [Move; MAX_MOVES],
    moves_len: usize,
}

impl MoveGen {
    pub fn from_board(board: &Board) -> Self {
        let jumpers = board.bitboards.get_jumpers_white();
        if jumpers > 0 {
            let (moves, moves_len) = Self::get_jumps(jumpers, board);
            MoveGen { moves, moves_len }
        } else {
            let movers = board.bitboards.get_movers_white();
            let mut moves: [Move; MAX_MOVES] = [Default::default(); MAX_MOVES];
            moves[0] = Self::get_non_jumps(movers, board);
            MoveGen {
                moves,
                moves_len: 1,
            }
        }
    }

    pub fn get_moves(&self) -> &[Move] {
        &self.moves[0..self.moves_len]
    }

    fn get_jumps(jumpers: u32, board: &Board) -> ([Move; MAX_MOVES], usize) {
        todo!()
    }

    fn get_non_jumps(mut movers: u32, board: &Board) -> Move {
        while movers > 0 {}

        todo!()
    }
}

pub fn get_next_square() -> String {
    use next_square::FormatNextSquare;

    next_square::NEXT_SQUARE.format()
}

mod next_square {

    pub const NEXT_SQUARE: [Option<u32>; 32 * 4] = gen_next_square();

    pub trait FormatNextSquare {
        fn format(&self) -> String;
    }

    impl FormatNextSquare for [Option<u32>; 32 * 4] {
        fn format(&self) -> String {
            let mut out = String::from("  ");

            for (i, s) in self.iter().enumerate() {
                let sc = match s {
                    Some(s) => format!("{:02}", s),
                    None => "NN".to_string(),
                };
                out.push_str(format!("{}, ", sc).as_str());

                // End of row newline
                if i % 4 == 3 {
                    out.push('\n');
                    let row = i / 4;
                    if row % 2 != 0 {
                        out.push_str("  ");
                    }
                }

                // End of table newline
                if i % 32 == 31 {
                    out.push_str("\n  ");
                }
            }

            out
        }
    }

    const fn gen_next_square() -> [Option<u32>; 32 * 4] {
        let mut next_square = [None; 32 * 4];

        let ul = get_next_square_ul();
        let ur = get_next_square_ur();
        let dl = get_next_square_dl();
        let dr = get_next_square_dr();

        let mut table = 0;
        let mut table_idx = 0;
        let mut i = 0;
        while i < 32 * 4 {
            let table = if i < 32 {
                &ul
            } else if i < 32 * 2 {
                &ur
            } else if i < 32 * 3 {
                &dl
            } else {
                &dr
            };

            let table_idx = i % 32;
            next_square[i] = table[table_idx];

            i += 1;
        }

        next_square
    }

    const fn get_next_square_ul() -> [Option<u32>; 32] {
        let mut up_left = [None; 32];
        let mut i = 0;
        while i < 32 {
            let row = i / 4;

            if row == 0 {
                up_left[i] = None;
            } else if row % 2 == 0 {
                up_left[i] = Some(i as u32 - 4);
            } else {
                let col = i % 4;

                if col == 0 {
                    up_left[i] = None;
                } else {
                    up_left[i] = Some(i as u32 - 5);
                }
            }
            i += 1;
        }

        up_left
    }

    const fn get_next_square_ur() -> [Option<u32>; 32] {
        let mut up_right = [None; 32];
        let mut i = 0;
        while i < 32 {
            let row = i / 4;

            if row == 0 {
                up_right[i] = None;
            } else if row % 2 != 0 {
                up_right[i] = Some(i as u32 - 4);
            } else {
                let col = i % 4;

                if col == 3 {
                    up_right[i] = None;
                } else {
                    up_right[i] = Some(i as u32 - 3);
                }
            }
            i += 1;
        }

        up_right
    }

    const fn get_next_square_dl() -> [Option<u32>; 32] {
        let mut down_left = [None; 32];
        let mut i = 0;
        while i < 32 {
            let row = i / 4;

            if row == 7 {
                down_left[i] = None;
            } else if row % 2 == 0 {
                down_left[i] = Some(i as u32 + 4);
            } else {
                let col = i % 4;

                if col == 0 {
                    down_left[i] = None;
                } else {
                    down_left[i] = Some(i as u32 + 3);
                }
            }
            i += 1;
        }

        down_left
    }

    const fn get_next_square_dr() -> [Option<u32>; 32] {
        let mut down_right = [None; 32];
        let mut i = 0;
        while i < 32 {
            let row = i / 4;

            if row == 7 {
                down_right[i] = None;
            } else if row % 2 != 0 {
                down_right[i] = Some(i as u32 + 4);
            } else {
                let col = i % 4;

                if col == 3 {
                    down_right[i] = None;
                } else {
                    down_right[i] = Some(i as u32 + 5);
                }
            }
            i += 1;
        }

        down_right
    }
}
