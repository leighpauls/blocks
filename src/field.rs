use crate::position::{p, Coord, Pos};
use crate::render::{BlockRenderInstructions, DrawBlockType, RenderBlockInfo};
use crate::shapes::{MinoSet, Shape};
use crate::tetromino::Tetromino;

#[derive(Copy, Clone, PartialEq, Debug)]
enum FieldBlock {
    Empty,
    Occupied(Shape),
}

pub struct Field {
    blocks: [[FieldBlock; Self::GAME_HEIGHT as usize]; Self::WIDTH as usize],
}

pub trait CheckableField {
    fn is_open(&self, pos: Pos) -> bool;
}

pub struct PlayingFieldRenderBlocksIterator<'a> {
    field: &'a Field,
    controlled_minos: MinoSet,
    ghost_minos: MinoSet,
    next_pos: Pos,
}

pub struct PlayingFieldRenderBlocksInstructions<'a> {
    field: &'a Field,
    controlled: Tetromino,
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

    pub fn occupy(&mut self, pos: Pos, shape: Shape) {
        self.blocks[pos.x as usize][pos.y as usize] = FieldBlock::Occupied(shape);
    }

    pub fn remove_lines(&mut self) -> i32 {
        let mut result = 0;
        'row_loop: for y in (0..(Self::GAME_HEIGHT as usize)).rev() {
            for x in 0..(Self::WIDTH as usize) {
                if let FieldBlock::Empty = self.blocks[x][y] {
                    continue 'row_loop;
                }
            }
            result += 1;
            self.drop_lines_above(y);
        }
        result
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

impl<'a> Iterator for PlayingFieldRenderBlocksIterator<'a> {
    type Item = RenderBlockInfo;

    fn next(&mut self) -> Option<RenderBlockInfo> {
        if self.next_pos.y >= Field::VISIBLE_HEIGHT {
            return None;
        }
        let pos = self.next_pos;
        let result = Some(RenderBlockInfo {
            pos: pos,
            block_type: if self.controlled_minos.contains(pos) {
                DrawBlockType::Occupied(self.controlled_minos.shape())
            } else if self.ghost_minos.contains(pos) {
                DrawBlockType::GhostPiece(self.ghost_minos.shape())
            } else if pos.y >= Field::PLAYING_BOUNDARY_HEIGHT {
                DrawBlockType::OutOfPlay
            } else {
                match self.field.blocks[pos.x as usize][pos.y as usize] {
                    FieldBlock::Empty => DrawBlockType::Empty,
                    FieldBlock::Occupied(shape) => DrawBlockType::Occupied(shape),
                }
            },
        });

        self.next_pos = self.next_pos + p(1, 0);
        if self.next_pos.x >= Field::WIDTH {
            self.next_pos = p(0, self.next_pos.y + 1);
        }
        result
    }
}

impl<'a> PlayingFieldRenderBlocksInstructions<'a> {
    pub fn new(field: &'a Field, controlled: Tetromino) -> Self {
        Self {
            field: field,
            controlled: controlled,
        }
    }
}

impl<'a> BlockRenderInstructions<PlayingFieldRenderBlocksIterator<'a>>
    for PlayingFieldRenderBlocksInstructions<'a>
{
    fn height_blocks(&self) -> Coord {
        Field::VISIBLE_HEIGHT
    }
    fn width_blocks(&self) -> Coord {
        Field::WIDTH
    }

    fn blocks(&self) -> PlayingFieldRenderBlocksIterator<'a> {
        PlayingFieldRenderBlocksIterator::<'a> {
            field: self.field,
            controlled_minos: self.controlled.to_minos(),
            ghost_minos: self.controlled.hard_drop(self.field).to_minos(),
            next_pos: p(0, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_at() {
        let mut f = Field::new();
        f.occupy(Pos::new(2, 3), Shape::O);

        assert!(f.is_open(Pos::new(1, 1)));
        assert!(!f.is_open(Pos::new(2, 3)));
    }

    #[test]
    fn is_open() {
        let mut f = Field::new();
        f.occupy(Pos::new(2, 3), Shape::O);

        assert_eq!(true, f.is_open(Pos::new(0, 0)));

        assert_eq!(true, f.is_open(Pos::new(2, 2)));
        assert_eq!(false, f.is_open(Pos::new(2, 3)));

        assert_eq!(false, f.is_open(Pos::new(-1, 0)));
        assert_eq!(false, f.is_open(Pos::new(0, -1)));

        assert_eq!(false, f.is_open(Pos::new(Field::WIDTH, 0)));
        assert_eq!(false, f.is_open(Pos::new(0, Field::GAME_HEIGHT)));
    }
}
