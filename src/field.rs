use crate::position::{Coord, Pos};
use crate::tetromino::CheckField;

pub const WIDTH: Coord = 10;
pub const GAME_HEIGHT: Coord = 40;
pub const VISIBLE_HEIGHT: Coord = 22;
pub const PLAYING_BOUNDARY_HEIGHT: Coord = 20;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FieldBlock {
    Empty,
    Occupied,
}

pub struct Field {
    blocks: [[FieldBlock; GAME_HEIGHT as usize]; WIDTH as usize],
}

impl Field {
    pub fn new() -> Field {
        Field {
            blocks: [[FieldBlock::Empty; GAME_HEIGHT as usize]; WIDTH as usize],
        }
    }

    pub fn at(&self, pos: Pos) -> FieldBlock {
        self.blocks[pos.x as usize][pos.y as usize]
    }

    pub fn set(&mut self, pos: Pos, value: FieldBlock) {
        self.blocks[pos.x as usize][pos.y as usize] = value;
    }

    pub fn remove_lines(&mut self) {
        'row_loop: for y in (0..(GAME_HEIGHT as usize)).rev() {
            for x in 0..(WIDTH as usize) {
                if let FieldBlock::Empty = self.blocks[x][y] {
                    continue 'row_loop;
                }
            }

            self.drop_lines_above(y);
        }
    }

    fn drop_lines_above(&mut self, row: usize) {
        for y in (row + 1)..(GAME_HEIGHT as usize) {
            for x in 0..(WIDTH as usize) {
                self.blocks[x][y - 1] = self.blocks[x][y];
            }
        }
        for x in 0..(WIDTH as usize) {
            self.blocks[x][(GAME_HEIGHT - 1) as usize] = FieldBlock::Empty;
        }
    }
}

impl CheckField for Field {
    fn is_open(&self, pos: Pos) -> bool {
        pos.x >= 0
            && pos.x < WIDTH
            && pos.y >= 0
            && pos.y < GAME_HEIGHT
            && self.at(pos) == FieldBlock::Empty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_at() {
        let mut f = Field::new();
        f.set(Pos::new(2, 3), FieldBlock::Occupied);

        assert_eq!(f.at(Pos::new(1, 1)), FieldBlock::Empty);
        assert_eq!(f.at(Pos::new(2, 3)), FieldBlock::Occupied);
    }

    #[test]
    fn is_open() {
        let mut f = Field::new();
        f.set(Pos::new(2, 3), FieldBlock::Occupied);

        assert_eq!(true, f.is_open(Pos::new(0, 0)));

        assert_eq!(true, f.is_open(Pos::new(2, 2)));
        assert_eq!(false, f.is_open(Pos::new(2, 3)));

        assert_eq!(false, f.is_open(Pos::new(-1, 0)));
        assert_eq!(false, f.is_open(Pos::new(0, -1)));

        assert_eq!(false, f.is_open(Pos::new(WIDTH, 0)));
        assert_eq!(false, f.is_open(Pos::new(0, GAME_HEIGHT)));
    }
}
