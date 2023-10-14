// Black
//   00  01  02  03
// 04  05  06  07
//   08  09  10  11
// 12  13  14  15
//   16  17  18  19
// 20  21  22  23
//   24  25  26  27
// 28  29  30  31
// White

const MASK_W3: u32 = 3772834016; //Sq+3 legal
const MASK_W5: u32 = 117901063; //Sq+5 legal
const MASK_B3: u32 = 117901056; //Sq-3 legal
const MASK_B5: u32 = 3772834016; //Sq-5 legal

const NEXT_SQUARE: [Option<u32>; 32 * 4] = gen_next_square();

trait FormatNextSquare {
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

pub fn get_next_square() -> String {
    NEXT_SQUARE.format()
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

#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Move {
    pub src: u8,
    pub dst: u8,
    jumps_len: u8,
    jumps_dirs: [Direction; 9],
}

impl Move {
    pub fn new(src: u8, dst: u8, jumps: &[Direction]) -> Self {
        let jumps_len = jumps.len() as u8;
        let mut jumps_dirs = [Direction::UpLeft; 9];
        jumps
            .iter()
            .enumerate()
            .for_each(|(i, j)| jumps_dirs[i] = *j);

        Move {
            src,
            dst,
            jumps_len,
            jumps_dirs,
        }
    }

    pub fn jumps(&self) -> &[Direction] {
        &self.jumps_dirs[0..self.jumps_len as usize]
    }
}

// enum MoverType {
//     Move(u32),
//     Jump(u32),
// }
//
// impl MoverType {
//     pub fn from(board: &Board) -> Self {
//         let jumpers = board.bitboards.get_jumpers_white();
//         if jumpers > 0 {
//             MoverType::Jump(jumpers)
//         } else {
//             let movers = board.bitboards.get_movers_white();
//             MoverType::Move(movers)
//         }
//     }
// }

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

// impl Iterator for MoveGen {
//     type Item = Move;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let square =
//     }
// }

// pub struct Piece {
//     pub color: Color,
//     pub king: bool,
// }

pub struct Board {
    pub bitboards: Bitboards,
    pub to_move: Color,
}

pub struct Bitboards {
    pub black: u32,
    pub white: u32,
    pub kings: u32,
}

impl Bitboards {
    fn get_movers_white(&self) -> u32 {
        let not_occ = !(self.white | self.black);
        let white_kings = self.white & self.kings;
        let mut movers = (not_occ << 4) & self.white;
        movers |= ((not_occ & MASK_W3) << 3) & self.white;
        movers |= ((not_occ & MASK_W5) << 5) & self.white;
        if white_kings > 0 {
            movers |= (not_occ >> 4) & white_kings;
            movers |= ((not_occ & MASK_B3) >> 3) & white_kings;
            movers |= ((not_occ & MASK_B5) >> 5) & white_kings;
        }

        movers
    }

    fn get_jumpers_white(&self) -> u32 {
        let not_occ = !(self.white | self.black);
        let white_kings = self.white & self.kings;
        let mut movers = 0u32;
        let temp = (not_occ << 4) & self.black;
        if temp > 0 {
            movers |= (((temp & MASK_W3) << 3) | ((temp & MASK_W5) << 5)) & self.white
        }
        let temp = (((not_occ & MASK_W3) << 3) | ((not_occ & MASK_W5) << 5)) & self.black;
        movers |= (temp << 4) & self.white;
        if white_kings > 0 {
            let temp = (not_occ >> 4) & self.black;
            if temp > 0 {
                movers |= (((temp & MASK_B3) >> 3) | ((temp & MASK_B5) >> 5)) & white_kings;
            }
            let temp = (((not_occ & MASK_B3) >> 3) | ((not_occ & MASK_B5) >> 5)) & self.black;
            if temp > 0 {
                movers |= (temp >> 4) & white_kings
            }
        }

        movers
    }

    pub fn print(&self) {
        for row in 0..8 {
            for col in 0..4 {
                let idx = (row * 4) + col;
                let mask = 1 << idx;
                let king = self.kings & mask > 0;

                if self.white & mask > 0 {
                    print!("w");
                    continue;
                } else if self.black & mask > 0 {
                    print!("b");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    pub fn format(&self) -> String {
        let mut out = "   A   B   C   D   E   F   G   H   \n".to_string();
        out.push_str(" ┌───┬───┬───┬───┬───┬───┬───┬───┐\n");
        for row in 0..15 {
            if row % 2 == 0 {
                // Convert display row into board row
                let row = row / 2;
                out.push_str(format!("{}│", row + 1).as_str());
                for col in 0..8 {
                    if (row + col) % 2 != 0 {
                        let idx = (row * 4) + col / 2;
                        let mask = 1 << idx;
                        let king = self.kings & mask > 0;
                        let color = if self.white & mask > 0 {
                            Some(Color::White)
                        } else if self.black & mask > 0 {
                            Some(Color::Black)
                        } else {
                            None
                        };

                        let piece = match (color, king) {
                            (None, _) => ' ',
                            (Some(Color::White), false) => '⛀',
                            (Some(Color::White), true) => '⛁',
                            (Some(Color::Black), false) => '⛂',
                            (Some(Color::Black), true) => '⛃',
                        };

                        out.push_str(format!(" {} │", piece).as_str());
                    } else {
                        out.push_str("   │");
                    }
                }
            } else {
                out.push_str(" ├───┼───┼───┼───┼───┼───┼───┼───┤");
            }
            out.push('\n');
        }
        out.push_str(" └───┴───┴───┴───┴───┴───┴───┴───┘");

        out
    }
}

#[cfg(test)]
mod tests {
    use crate::Bitboards;

    // #[test]
    // fn check_movers_white() {
    //     let board = Board {
    //         black: 0,
    //         white: 1145324611,
    //         kings: 2,
    //     };
    //
    //     println!("check_movers_white:\n{}", board.format());
    //
    //     let movers = 67108866;
    //
    //     assert_eq!(board.get_movers_white(), movers);
    // }

    #[test]
    fn check_movers_white_starting() {
        let board = Bitboards {
            black: 0,
            white: 4293918720,
            kings: 0,
        };

        println!("check_movers_white_starting:\n{}", board.format());

        let movers = 15728640;

        assert_eq!(board.get_movers_white(), movers);
    }

    #[test]
    fn check_get_jumpers_white() {
        let board = Bitboards {
            black: 71434240,
            white: 1109393408,
            kings: 0,
        };

        println!("check_get_jumpers_white:\n{}", board.format());

        let movers = 1109393408;

        assert_eq!(board.get_jumpers_white(), movers);
    }
}
