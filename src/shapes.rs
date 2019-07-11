use crate::field::{CheckableField, Field};
use crate::position::Coord;
use crate::position::{p, Pos, RotateDir, Rotations};
use crate::render::{BlockRenderInstructions, DrawBlockType, RenderBlockInfo};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shape {
    I,
    O,
    J,
    L,
    S,
    Z,
    T,
}

#[derive(Clone)]
pub struct MinoSet {
    minos: [Pos; 4],
    shape: Shape,
}

pub struct PreviewRenderBlocksIterator {
    minos: MinoSet,
    i: usize,
}

pub trait ShapeDef {
    fn to_minos(&self, rotation: Rotations, root_pos: Pos) -> MinoSet;
    fn wall_kick_offsets(&self, initial_rot: Rotations, rot_dir: RotateDir) -> Vec<Pos>;
}

impl MinoSet {
    pub fn shape(&self) -> Shape {
        self.shape
    }

    pub fn contains(&self, p: Pos) -> bool {
        self.minos.contains(&p)
    }

    pub fn apply_to_field(&self, field: &mut Field) {
        for mino in self.minos.iter() {
            field.occupy(*mino, self.shape);
        }
    }

    pub fn is_valid(&self, field: &CheckableField) -> bool {
        for mino in self.minos.iter() {
            if !field.is_open(*mino) {
                return false;
            }
        }
        true
    }
}

impl ShapeDef for Shape {
    fn to_minos(&self, rotation: Rotations, root_pos: Pos) -> MinoSet {
        let p = self.positions(rotation);
        MinoSet {
            minos: [
                root_pos + p[0],
                root_pos + p[1],
                root_pos + p[2],
                root_pos + p[3],
            ],
            shape: *self,
        }
    }

    fn wall_kick_offsets(&self, initial_rot: Rotations, rot_dir: RotateDir) -> Vec<Pos> {
        use RotateDir::*;
        use Rotations::*;
        use Shape::*;
        match (self, rot_dir) {
            (O, _) => vec![p(0, 0)],

            (I, CW) => match initial_rot {
                Zero => vec![p(0, 0), p(-2, 0), p(1, 0), p(-2, -1), p(1, 2)],
                One => vec![p(0, 0), p(-1, 0), p(2, 0), p(-1, 2), p(2, -1)],
                Two => vec![p(0, 0), p(2, 0), p(-1, 0), p(2, 1), p(-1, -2)],
                Three => vec![p(0, 0), p(1, 0), p(-2, 0), p(1, -2), p(-2, 1)],
            },
            (I, CCW) => match initial_rot {
                Zero => vec![p(0, 0), p(-1, 0), p(2, 0), p(-1, 2), p(2, -1)],
                One => vec![p(0, 0), p(2, 0), p(-1, 0), p(2, 1), p(-1, -2)],
                Two => vec![p(0, 0), p(1, 0), p(-2, 0), p(1, -2), p(-2, 1)],
                Three => vec![p(0, 0), p(-2, 0), p(1, 0), p(-2, -1), p(1, 2)],
            },
            (_, CW) => match initial_rot {
                Zero => vec![p(0, 0), p(-1, 0), p(-1, 1), p(0, -2), p(-1, -2)],
                One => vec![p(0, 0), p(1, 0), p(1, -1), p(0, 2), p(1, 2)],
                Two => vec![p(0, 0), p(1, 0), p(1, 1), p(0, -2), p(1, -2)],
                Three => vec![p(0, 0), p(-1, 0), p(-1, -1), p(0, 2), p(-1, 2)],
            },
            (_, CCW) => match initial_rot {
                Zero => vec![p(0, 0), p(1, 0), p(1, 1), p(0, -2), p(1, -2)],
                One => vec![p(0, 0), p(1, 0), p(1, -1), p(0, 2), p(1, 2)],
                Two => vec![p(0, 0), p(-1, 0), p(-1, 1), p(0, -2), p(-1, -2)],
                Three => vec![p(0, 0), p(-1, 0), p(-1, -1), p(0, 2), p(-1, 2)],
            },
        }
    }
}

impl Shape {
    fn positions(&self, rotation: Rotations) -> [Pos; 4] {
        use Rotations::*;
        use Shape::*;
        match self {
            O => [p(1, 2), p(2, 2), p(1, 3), p(2, 3)],
            I => match rotation {
                Zero => [p(0, 2), p(1, 2), p(2, 2), p(3, 2)],
                One => [p(2, 0), p(2, 1), p(2, 2), p(2, 3)],
                Two => [p(0, 1), p(1, 1), p(2, 1), p(3, 1)],
                Three => [p(1, 0), p(1, 1), p(1, 2), p(1, 3)],
            },
            J => match rotation {
                Zero => [p(0, 3), p(0, 2), p(1, 2), p(2, 2)],
                One => [p(1, 1), p(1, 2), p(1, 3), p(2, 3)],
                Two => [p(0, 2), p(1, 2), p(2, 2), p(2, 1)],
                Three => [p(0, 1), p(1, 1), p(1, 2), p(1, 3)],
            },
            L => match rotation {
                Zero => [p(2, 3), p(0, 2), p(1, 2), p(2, 2)],
                One => [p(1, 1), p(1, 2), p(1, 3), p(2, 1)],
                Two => [p(0, 2), p(1, 2), p(2, 2), p(0, 1)],
                Three => [p(0, 3), p(1, 1), p(1, 2), p(1, 3)],
            },
            S => match rotation {
                Zero => [p(0, 2), p(1, 2), p(1, 3), p(2, 3)],
                One => [p(1, 3), p(1, 2), p(2, 2), p(2, 1)],
                Two => [p(0, 1), p(1, 1), p(1, 2), p(2, 2)],
                Three => [p(0, 3), p(0, 2), p(1, 2), p(1, 1)],
            },
            Z => match rotation {
                Zero => [p(0, 3), p(1, 3), p(1, 2), p(2, 2)],
                One => [p(1, 1), p(1, 2), p(2, 2), p(2, 3)],
                Two => [p(0, 2), p(1, 2), p(1, 1), p(2, 1)],
                Three => [p(0, 1), p(0, 2), p(1, 2), p(1, 3)],
            },
            T => match rotation {
                Zero => [p(0, 2), p(1, 2), p(1, 3), p(2, 2)],
                One => [p(1, 3), p(1, 2), p(1, 1), p(2, 2)],
                Two => [p(0, 2), p(1, 2), p(2, 2), p(1, 1)],
                Three => [p(0, 2), p(1, 1), p(1, 2), p(1, 3)],
            },
        }
    }
}

impl BlockRenderInstructions<PreviewRenderBlocksIterator> for Shape {
    fn blocks(&self) -> PreviewRenderBlocksIterator {
        PreviewRenderBlocksIterator {
            minos: self.to_minos(Rotations::Zero, p(0, 0)),
            i: 0,
        }
    }

    fn height_blocks(&self) -> Coord {
        2
    }
    fn width_blocks(&self) -> Coord {
        4
    }
}

impl Iterator for PreviewRenderBlocksIterator {
    type Item = RenderBlockInfo;
    fn next(&mut self) -> Option<RenderBlockInfo> {
        if self.i < 4 {
            let result = RenderBlockInfo {
                pos: self.minos.minos[self.i],
                block_type: DrawBlockType::Occupied(self.minos.shape),
            };
            self.i += 1;
            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest2::prelude::*;

    #[test]
    fn to_minos() {
        let minos = Shape::I.to_minos(Rotations::One, p(1, 2)).minos;
        assert_that!(
            &minos,
            contains(vec![p(3, 2), p(3, 3), p(3, 4), p(3, 5)]).exactly()
        );
    }
}
