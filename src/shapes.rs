use crate::position::{p, Pos, RelativePoses, RotateDir, Rotations};

#[derive(Copy, Clone)]
pub enum Shape {
    I,
    O,

    #[cfg(test)]
    TestShape,
}

pub fn shape_positions(shape: Shape, rotations: Rotations) -> RelativePoses {
    use Rotations::*;
    use Shape::*;
    match (shape, rotations) {
        (O, _) => [p(1, 1), p(2, 1), p(1, 2), p(2, 2)],

        (I, Zero) => [p(0, 2), p(1, 2), p(2, 2), p(3, 2)],
        (I, One) => [p(2, 0), p(2, 1), p(2, 2), p(2, 3)],
        (I, Two) => [p(0, 1), p(1, 1), p(2, 1), p(3, 1)],
        (I, Three) => [p(1, 0), p(1, 1), p(1, 2), p(1, 3)],

        #[cfg(test)]
        (TestShape, _) => [p(0, 0); 4],
    }
}

pub fn wall_kick_offsets(shape: Shape, initial_rot: Rotations, rot_dir: RotateDir) -> Vec<Pos> {
    use RotateDir::*;
    use Rotations::*;
    use Shape::*;
    match (shape, initial_rot, rot_dir) {
        (O, _, _) => vec![p(0, 0)],

        (I, Zero, CW) => vec![p(0, 0), p(-2, 0), p(1, 0), p(-2, -1), p(1, 2)],
        (I, One, CW) => vec![p(0, 0), p(-1, 0), p(2, 0), p(-1, 2), p(2, -1)],
        (I, Two, CW) => vec![p(0, 0), p(2, 0), p(-1, 0), p(2, 1), p(-1, -2)],
        (I, Three, CW) => vec![p(0, 0), p(1, 0), p(-2, 0), p(1, -2), p(-2, 1)],

        (I, Zero, CCW) => vec![p(0, 0), p(-1, 0), p(2, 0), p(-1, 2), p(2, -1)],
        (I, One, CCW) => vec![p(0, 0), p(2, 0), p(-1, 0), p(2, 1), p(-1, -2)],
        (I, Two, CCW) => vec![p(0, 0), p(1, 0), p(-2, 0), p(1, -2), p(-2, 1)],
        (I, Three, CCW) => vec![p(0, 0), p(-2, 0), p(1, 0), p(-2, -1), p(1, 2)],

        #[cfg(test)]
        (TestShape, _, _) => vec![p(0, 0)],
    }
}
