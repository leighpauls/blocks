use crate::position::{p, RelativePoses};

#[derive(Copy, Clone)]
pub enum Shape {
    I,
    O,

    #[cfg(test)]
    TestShape,
}

// Number of clockwise rotations
pub enum Rotations {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
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
