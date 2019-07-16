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
    game_minos: GameMinos,
    next_pos: Pos,
}

pub struct PlayingFieldRenderBlocksInstructions<'a> {
    field: &'a Field,
    controlled: Tetromino,
}

enum GameMinos {
    Controlled(ControlMinos),
    Clearing(Vec<Coord>),
}

struct ControlMinos {
    controlled: MinoSet,
    ghost: MinoSet,
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
        *self.bp_mut(pos) = FieldBlock::Occupied(shape);
    }

    pub fn find_lines(&self) -> Vec<Coord> {
        let mut result = vec![];
        'row_loop: for y in (0..Self::GAME_HEIGHT).rev() {
            for x in 0..Self::WIDTH {
                if let FieldBlock::Empty = self.b(x, y) {
                    continue 'row_loop;
                }
            }
            result.push(y);
        }
        result
    }

    pub fn remove_lines(&mut self, lines: &Vec<Coord>) {
        for y in lines {
            self.drop_lines_above(*y);
        }
    }

    fn drop_lines_above(&mut self, row: Coord) {
        for y in (row + 1)..Self::GAME_HEIGHT {
            for x in 0..Self::WIDTH {
                *self.b_mut(x, y - 1) = self.b(x, y);
            }
        }
        for x in 0..Self::WIDTH {
            *self.b_mut(x, Self::GAME_HEIGHT - 1) = FieldBlock::Empty;
        }
    }

    fn bp(&self, p: Pos) -> FieldBlock {
        self.b(p.x, p.y)
    }
    fn b(&self, x: Coord, y: Coord) -> FieldBlock {
        self.blocks[x as usize][y as usize]
    }

    fn bp_mut(&mut self, p: Pos) -> &mut FieldBlock {
        self.b_mut(p.x, p.y)
    }
    fn b_mut(&mut self, x: Coord, y: Coord) -> &mut FieldBlock {
        &mut self.blocks[x as usize][y as usize]
    }
}

impl CheckableField for Field {
    fn is_open(&self, pos: Pos) -> bool {
        pos.x >= 0
            && pos.x < Self::WIDTH
            && pos.y >= 0
            && pos.y < Self::GAME_HEIGHT
            && self.bp(pos) == FieldBlock::Empty
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
            block_type: self.select_block_type(pos),
        });

        self.next_pos = self.next_pos + p(1, 0);
        if self.next_pos.x >= Field::WIDTH {
            self.next_pos = p(0, self.next_pos.y + 1);
        }
        result
    }
}

impl<'a> PlayingFieldRenderBlocksIterator<'a> {
    fn select_block_type(&self, pos: Pos) -> DrawBlockType {
        if let GameMinos::Controlled(minos) = &self.game_minos {
            if minos.controlled.contains(pos) {
                return DrawBlockType::Occupied(minos.controlled.shape());
            } else if minos.ghost.contains(pos) {
                return DrawBlockType::GhostPiece(minos.ghost.shape());
            }
        } else if let GameMinos::Clearing(lines) = &self.game_minos {
            if lines.contains(&pos.y) {
                return DrawBlockType::ClearingLine;
            }
        }
        if pos.y >= Field::PLAYING_BOUNDARY_HEIGHT {
            DrawBlockType::OutOfPlay
        } else {
            match self.field.bp(pos) {
                FieldBlock::Empty => DrawBlockType::Empty,
                FieldBlock::Occupied(shape) => DrawBlockType::Occupied(shape),
            }
        }
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
            game_minos: GameMinos::Controlled(ControlMinos {
                controlled: self.controlled.to_minos(),
                ghost: self.controlled.hard_drop(self.field).to_minos(),
            }),
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
