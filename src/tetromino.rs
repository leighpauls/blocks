use crate::position::{p, Pos, RotateDir, Rotations, ShiftDir};
use crate::shapes::{MinoSet, Shape, ShapeDef};

#[derive(Clone, Copy, Debug)]
pub struct Tetromino {
    root_pos: Pos,
    shape: Shape,
    rotation: Rotations,
}

pub trait CheckField {
    fn is_open(&self, pos: Pos) -> bool;

    fn are_open(&self, positions: &[Pos; 4]) -> bool {
        for p in positions.iter() {
            if !self.is_open(*p) {
                return false;
            }
        }
        true
    }
}

impl Tetromino {
    pub fn new(p: Pos, s: Shape) -> Tetromino {
        Tetromino {
            root_pos: p,
            shape: s,
            rotation: Rotations::Zero,
        }
    }

    pub fn to_minos(&self) -> MinoSet {
        self.shape.to_minos(self.rotation, self.root_pos)
    }

    pub fn hard_drop(&self, field: &CheckField) -> Tetromino {
        let mut result = *self;
        loop {
            if let Some(new) = result.try_down(field) {
                result = new;
            } else {
                return result;
            }
        }
    }

    pub fn try_down(&self, field: &CheckField) -> Option<Tetromino> {
        Tetromino {
            root_pos: self.root_pos + p(0, -1),
            ..*self
        }
        .if_valid(field)
    }

    pub fn try_shift(&self, dir: ShiftDir, field: &CheckField) -> Option<Tetromino> {
        Tetromino {
            root_pos: self.root_pos + dir,
            ..*self
        }
        .if_valid(field)
    }

    pub fn try_rotate(&self, dir: RotateDir, field: &CheckField) -> Option<Tetromino> {
        let new_rotation = self.rotation + dir;

        'kick: for kick_offset in self.shape.wall_kick_offsets(self.rotation, dir).into_iter() {
            let new = Tetromino {
                root_pos: self.root_pos + kick_offset,
                rotation: new_rotation,
                ..*self
            };

            if !field.are_open(&new.to_minos().minos) {
                continue 'kick;
            }
            return Some(new);
        }
        None
    }

    fn if_valid(self, field: &CheckField) -> Option<Self> {
        if field.are_open(&self.to_minos().minos) {
            Some(self)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest2::prelude::*;

    mock_trait!(MockCheckField, is_open(Pos) -> bool);
    impl CheckField for MockCheckField {
        mock_method!(is_open(&self, pos: Pos) -> bool);
    }

    #[test]
    fn hard_drop() {
        let mock_field = MockCheckField::default();
        mock_field.is_open.return_value_for(p(0, -5), false);
        mock_field.is_open.return_value(true);

        let t = Tetromino::new(p(0, 0), Shape::I);
        let result = t.hard_drop(&mock_field);

        assert_eq!(p(0, -6), result.root_pos);
    }

    #[test]
    fn shift() {
        let mock_field = MockCheckField::default();
        mock_field.is_open.return_value_for(p(-1, 2), false);
        mock_field.is_open.return_value(true);

        let t = Tetromino::new(p(0, 0), Shape::I);
        assert_that!(t.try_shift(ShiftDir::Left, &mock_field), not(some()));

        let right_result = t.try_shift(ShiftDir::Right, &mock_field);
        assert_eq!(right_result.expect("").root_pos, p(1, 0));
    }
}
