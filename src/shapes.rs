use crate::position::{p, Pos, RelativePoses, RotateDir, Rotations};
use num_traits::FromPrimitive;
use rand::random;

const NUM_SHAPES: usize = 2;

pub trait ShapeDef {
    fn positions(&self, rotation: Rotations) -> RelativePoses;
    fn wall_kick_offsets(&self, initial_rot: Rotations, rot_dir: RotateDir) -> Vec<Pos>;
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum Shape {
    I,
    O,
}

impl ShapeDef for Shape {
    fn positions(&self, rotation: Rotations) -> RelativePoses {
        use Rotations::*;
        use Shape::*;
        match (self, rotation) {
            (O, _) => [p(1, 2), p(2, 2), p(1, 3), p(2, 3)],

            (I, Zero) => [p(0, 2), p(1, 2), p(2, 2), p(3, 2)],
            (I, One) => [p(2, 0), p(2, 1), p(2, 2), p(2, 3)],
            (I, Two) => [p(0, 1), p(1, 1), p(2, 1), p(3, 1)],
            (I, Three) => [p(1, 0), p(1, 1), p(1, 2), p(1, 3)],
        }
    }

    fn wall_kick_offsets(&self, initial_rot: Rotations, rot_dir: RotateDir) -> Vec<Pos> {
        use RotateDir::*;
        use Rotations::*;
        use Shape::*;
        match (self, initial_rot, rot_dir) {
            (O, _, _) => vec![p(0, 0)],

            (I, Zero, CW) => vec![p(0, 0), p(-2, 0), p(1, 0), p(-2, -1), p(1, 2)],
            (I, One, CW) => vec![p(0, 0), p(-1, 0), p(2, 0), p(-1, 2), p(2, -1)],
            (I, Two, CW) => vec![p(0, 0), p(2, 0), p(-1, 0), p(2, 1), p(-1, -2)],
            (I, Three, CW) => vec![p(0, 0), p(1, 0), p(-2, 0), p(1, -2), p(-2, 1)],

            (I, Zero, CCW) => vec![p(0, 0), p(-1, 0), p(2, 0), p(-1, 2), p(2, -1)],
            (I, One, CCW) => vec![p(0, 0), p(2, 0), p(-1, 0), p(2, 1), p(-1, -2)],
            (I, Two, CCW) => vec![p(0, 0), p(1, 0), p(-2, 0), p(1, -2), p(-2, 1)],
            (I, Three, CCW) => vec![p(0, 0), p(-2, 0), p(1, 0), p(-2, -1), p(1, 2)],
        }
    }
}

impl Shape {
    pub fn random() -> Shape {
        Shape::from_usize(random::<usize>() % NUM_SHAPES).expect("Unexpected shape enum value")
    }
}
