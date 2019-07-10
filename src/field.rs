use crate::position::{Coord, Pos};

#[derive(Copy, Clone, PartialEq, Debug)]
enum FieldBlock {
    Empty,
    Occupied,
}

pub struct Field {
    blocks: [[FieldBlock; Self::GAME_HEIGHT as usize]; Self::WIDTH as usize],
}

pub trait CheckableField {
    fn is_open(&self, pos: Pos) -> bool;
}

impl Field {
    pub const WIDTH: Coord = 10;
    pub const GAME_HEIGHT: Coord = 40;
    pub const VISIBLE_HEIGHT: Coord = 22;
    pub const PLAYING_BOUNDARY_HEIGHT: Coord = 20;

    pub fn new() -> Field {
        Field {
            blocks: [[FieldBlock::Empty; Self::GAME_HEIGHT as usize]; Self::WIDTH as usize],
        }
    }

    pub fn occupy(&mut self, pos: Pos) {
        self.blocks[pos.x as usize][pos.y as usize] = FieldBlock::Occupied;
    }

    pub fn remove_lines(&mut self) {
        'row_loop: for y in (0..(Self::GAME_HEIGHT as usize)).rev() {
            for x in 0..(Self::WIDTH as usize) {
                if let FieldBlock::Empty = self.blocks[x][y] {
                    continue 'row_loop;
                }
            }

            self.drop_lines_above(y);
        }
    }

    fn drop_lines_above(&mut self, row: usize) {
        for y in (row + 1)..(Self::GAME_HEIGHT as usize) {
            for x in 0..(Self::WIDTH as usize) {
                self.blocks[x][y - 1] = self.blocks[x][y];
            }
        }
        for x in 0..(Self::WIDTH as usize) {
            self.blocks[x][(Self::GAME_HEIGHT - 1) as usize] = FieldBlock::Empty;
        }
    }
}

impl CheckableField for Field {
    fn is_open(&self, pos: Pos) -> bool {
        pos.x >= 0
            && pos.x < Self::WIDTH
            && pos.y >= 0
            && pos.y < Self::GAME_HEIGHT
            && self.blocks[pos.x as usize][pos.y as usize] == FieldBlock::Empty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_at() {
        let mut f = Field::new();
        f.occupy(Pos::new(2, 3));

        assert!(f.is_open(Pos::new(1, 1)));
        assert!(!f.is_open(Pos::new(2, 3)));
    }

    #[test]
    fn is_open() {
        let mut f = Field::new();
        f.occupy(Pos::new(2, 3));

        assert_eq!(true, f.is_open(Pos::new(0, 0)));

        assert_eq!(true, f.is_open(Pos::new(2, 2)));
        assert_eq!(false, f.is_open(Pos::new(2, 3)));

        assert_eq!(false, f.is_open(Pos::new(-1, 0)));
        assert_eq!(false, f.is_open(Pos::new(0, -1)));

        assert_eq!(false, f.is_open(Pos::new(Field::WIDTH, 0)));
        assert_eq!(false, f.is_open(Pos::new(0, Field::GAME_HEIGHT)));
    }
}
