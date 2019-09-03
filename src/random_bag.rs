use crate::shapes::Shape;
use alloc::collections::vec_deque::VecDeque;
use alloc::vec::Vec;
use getrandom;

pub struct RandomBag {
    remaining: Vec<Shape>,
    upcoming: VecDeque<Shape>,
}

const NUM_SHAPES: usize = 7;

const ALL_SHAPES: [Shape; NUM_SHAPES] = [
    Shape::I,
    Shape::O,
    Shape::J,
    Shape::L,
    Shape::S,
    Shape::Z,
    Shape::T,
];

const NUM_PREVIEWS: usize = 6;

impl RandomBag {
    pub fn new() -> RandomBag {
        let mut result = RandomBag {
            remaining: ALL_SHAPES.to_vec(),
            upcoming: VecDeque::with_capacity(NUM_PREVIEWS),
        };
        while result.upcoming.len() < NUM_PREVIEWS {
            result.fill_upcoming();
        }
        result
    }

    pub fn previews(&self) -> Vec<Shape> {
        self.upcoming.clone().into()
    }

    pub fn take_next(&mut self) -> Shape {
        self.fill_upcoming();
        self.upcoming
            .pop_front()
            .expect("Expected upcoming to have values")
    }

    fn fill_upcoming(&mut self) {
        let mut rand_byte: [u8; 1] = [0];
        getrandom::getrandom(&mut rand_byte).unwrap();
        self.upcoming.push_back(
            self.remaining
                .remove((rand_byte[0] as usize) % self.remaining.len()),
        );
        if self.remaining.is_empty() {
            self.remaining = ALL_SHAPES.to_vec();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest2::prelude::*;

    #[test]
    fn select_all_from_bag() {
        let mut r = RandomBag::new();
        let mut seen_shapes = vec![];
        for _ in 0..NUM_SHAPES {
            seen_shapes.push(r.take_next())
        }

        assert_that!(&seen_shapes, contains(ALL_SHAPES.to_vec()).exactly());
    }
}
