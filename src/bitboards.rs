use crate::Color;

const MASK_W3: u32 = 3772834016; //Sq+3 legal
const MASK_W5: u32 = 117901063; //Sq+5 legal
const MASK_B3: u32 = 117901056; //Sq-3 legal
const MASK_B5: u32 = 3772834016; //Sq-5 legal

pub struct Bitboards {
    pub black: u32,
    pub white: u32,
    pub kings: u32,
}

impl Bitboards {
    pub fn get_movers_white(&self) -> u32 {
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

    pub fn get_jumpers_white(&self) -> u32 {
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
    use super::Bitboards;

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
