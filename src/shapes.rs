use crate::position::{p, Pos, RelativePoses, RotateDir, Rotations};
use num_traits::FromPrimitive;
use rand::random;

pub trait ShapeDef {
    fn positions(&self, rotation: Rotations) -> RelativePoses;
    fn wall_kick_offsets(&self, initial_rot: Rotations, rot_dir: RotateDir) -> Vec<Pos>;
}

const NUM_SHAPES: usize = 7;

#[derive(Copy, Clone, FromPrimitive)]
pub enum Shape {
    I,
    O,
    J,
    L,
    S,
    Z,
    T,
}

impl ShapeDef for Shape {
    fn positions(&self, rotation: Rotations) -> RelativePoses {
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
    pub fn random() -> Shape {
        Shape::from_usize(random::<usize>() % NUM_SHAPES).expect("Unexpected shape enum value")
    }
}
