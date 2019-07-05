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
    match (shape, rotations) {
        (Shape::O, _) => [p(1, 1), p(2, 1), p(1, 2), p(2, 2)],
        (Shape::I, Rotations::Zero) => [p(0, 2), p(1, 2), p(2, 2), p(3, 2)],
        (Shape::I, _) => [p(0, 2), p(1, 2), p(2, 2), p(3, 2)],

        #[cfg(test)]
        (Shape::TestShape, _) => [p(0, 0); 4],
    }
}
