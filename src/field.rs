use crate::position::{p, Coord, Pos};

pub const WIDTH: Coord = 10;
pub const GAME_HEIGHT: Coord = 40;
pub const VISIBLE_HEIGHT: Coord = 22;
pub const PLAYING_BOUNDARY_HEIGHT: Coord = 20;

#[derive(Copy, Clone, PartialEq, Debug)]
enum FieldBlock {
    Empty,
    Occupied,
}

pub struct Field {
    blocks: [[FieldBlock; GAME_HEIGHT as usize]; WIDTH as usize],
}

pub trait CheckableField {
    fn is_open(&self, pos: Pos) -> bool;
}

pub struct VisibleBlock {
    pub pos: Pos,
    pub is_occupied: bool,
}

impl Field {
    pub fn iter(&self) -> impl Iterator<Item = VisibleBlock> + '_ {
        FieldIter {
            field: self,
            next_pos: p(0, 0),
        }
    }

    pub fn new() -> Field {
        Field {
            blocks: [[FieldBlock::Empty; GAME_HEIGHT as usize]; WIDTH as usize],
        }
    }

    pub fn occupy(&mut self, pos: Pos) {
        self.blocks[pos.x as usize][pos.y as usize] = FieldBlock::Occupied;
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

impl CheckableField for Field {
    fn is_open(&self, pos: Pos) -> bool {
        pos.x >= 0
            && pos.x < WIDTH
            && pos.y >= 0
            && pos.y < GAME_HEIGHT
            && self.at(pos) == FieldBlock::Empty
    }
}

trait IterableField {
    const WIDTH: Coord;
    const VISIBLE_HEIGHT: Coord;

    fn at(&self, pos: Pos) -> FieldBlock;
}

struct FieldIter<'a, T: IterableField> {
    field: &'a T,
    next_pos: Pos,
}

impl<'a, T: IterableField> Iterator for FieldIter<'a, T> {
    type Item = VisibleBlock;

    fn next(&mut self) -> Option<VisibleBlock> {
        if self.next_pos.y >= T::VISIBLE_HEIGHT {
            return None;
        }
        let result = Some(VisibleBlock {
            pos: self.next_pos,
            is_occupied: self.field.at(self.next_pos) == FieldBlock::Occupied,
        });

        self.next_pos = self.next_pos + p(1, 0);
        if self.next_pos.x >= T::WIDTH {
            self.next_pos = p(0, self.next_pos.y + 1);
        }
        result
    }
}

impl IterableField for Field {
    fn at(&self, pos: Pos) -> FieldBlock {
        self.blocks[pos.x as usize][pos.y as usize]
    }

    const WIDTH: Coord = WIDTH;
    const VISIBLE_HEIGHT: Coord = VISIBLE_HEIGHT;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_at() {
        let mut f = Field::new();
        f.occupy(Pos::new(2, 3));

        assert_eq!(f.at(Pos::new(1, 1)), FieldBlock::Empty);
        assert_eq!(f.at(Pos::new(2, 3)), FieldBlock::Occupied);
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

        assert_eq!(false, f.is_open(Pos::new(WIDTH, 0)));
        assert_eq!(false, f.is_open(Pos::new(0, GAME_HEIGHT)));
    }

    #[test]
    fn iter_field() {
        let f = Field::new();
        let all_blocks: Vec<VisibleBlock> = f.iter().collect();
        assert_eq!(10 * 22, all_blocks.len());
    }
}
