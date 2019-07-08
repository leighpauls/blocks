use crate::field::CheckableField;
use crate::position::{p, Pos, RotateDir, Rotations, ShiftDir};
use crate::shapes::{MinoSet, Shape, ShapeDef};

#[derive(Clone, Copy, Debug)]
pub struct Tetromino {
    root_pos: Pos,
    shape: Shape,
    rotation: Rotations,
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

    pub fn hard_drop(&self, field: &CheckableField) -> Tetromino {
        let mut result = *self;
        loop {
            if let Some(new) = result.try_down(field) {
                result = new;
            } else {
                return result;
            }
        }
    }

    pub fn try_down(&self, field: &CheckableField) -> Option<Tetromino> {
        Tetromino {
            root_pos: self.root_pos + p(0, -1),
            ..*self
        }
        .if_valid(field)
    }

    pub fn try_shift(&self, dir: ShiftDir, field: &CheckableField) -> Option<Tetromino> {
        Tetromino {
            root_pos: self.root_pos + dir,
            ..*self
        }
        .if_valid(field)
    }

    pub fn try_rotate(&self, dir: RotateDir, field: &CheckableField) -> Option<Tetromino> {
        let new_rotation = self.rotation + dir;

        'kick: for kick_offset in self.shape.wall_kick_offsets(self.rotation, dir).into_iter() {
            let new = Tetromino {
                root_pos: self.root_pos + kick_offset,
                rotation: new_rotation,
                ..*self
            };

            if !new.to_minos().is_valid(field) {
                continue 'kick;
            }
            return Some(new);
        }
        None
    }

    fn if_valid(self, field: &CheckableField) -> Option<Self> {
        if self.to_minos().is_valid(field) {
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

    mock_trait!(MockCheckableField, is_open(Pos) -> bool);
    impl CheckableField for MockCheckableField {
        mock_method!(is_open(&self, pos: Pos) -> bool);
    }

    #[test]
    fn hard_drop() {
        let mock_field = MockCheckableField::default();
        mock_field.is_open.return_value_for(p(0, -5), false);
        mock_field.is_open.return_value(true);

        let t = Tetromino::new(p(0, 0), Shape::I);
        let result = t.hard_drop(&mock_field);

        assert_eq!(p(0, -6), result.root_pos);
    }

    #[test]
    fn shift() {
        let mock_field = MockCheckableField::default();
        mock_field.is_open.return_value_for(p(-1, 2), false);
        mock_field.is_open.return_value(true);

        let t = Tetromino::new(p(0, 0), Shape::I);
        assert_that!(t.try_shift(ShiftDir::Left, &mock_field), not(some()));

        let right_result = t.try_shift(ShiftDir::Right, &mock_field);
        assert_eq!(right_result.expect("").root_pos, p(1, 0));
    }
}
