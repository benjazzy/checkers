const MASK_L3: u32 = 235802126; //Sq+3 legal
const MASK_L5: u32 = 7368816; //Sq+5 legal
const MASK_R3: u32 = 1886417008; //Sq-3 legal
const MASK_R5: u32 = 235802112; //Sq-5 legal

pub enum Color {
    White,
    Black,
}

pub struct Piece {
    pub color: Color,
    pub king: bool,
}

pub struct Board {
    pub black: u32,
    pub white: u32,
    pub kings: u32,
}

impl Board {
    fn get_movers_white(&self) -> u32 {
        let not_occ = !(self.white | self.black);
        let white_kings = self.white & self.kings;
        let mut movers = (not_occ << 4) & self.white;
        movers |= ((not_occ & MASK_L3) << 3) & self.white;
        movers |= ((not_occ & MASK_L5) << 5) & self.white;
        if white_kings > 0 {
            movers |= (not_occ >> 4) & white_kings;
            movers |= ((not_occ & MASK_R3) >> 3) & white_kings;
            movers |= ((not_occ & MASK_R5) >> 5) & white_kings;
        }

        movers
    }

    pub fn print(&self) {
        for row in (0..8).rev() {
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
        for row in (0..15).rev() {
            if row % 2 == 0 {
                // Convert display row into board row
                let row = row / 2;
                out.push_str(format!("{}│", row + 1).as_str());
                for col in 0..8 {
                    if (row + col) % 2 == 0 {
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
    use crate::Board;

    #[test]
    fn check_movers_white() {
        let board = Board {
            black: 0,
            white: 536870915,
            kings: 1,
        };

        println!("{}", board.format());

        let movers = 536870913;

        assert_eq!(board.get_movers_white(), movers);
    }
}
