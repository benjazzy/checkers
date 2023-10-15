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

pub mod bitboards;
pub mod board;
pub mod direction;
pub mod move_gen;

use direction::Direction;

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
