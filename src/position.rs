use core::ops::Add;
use num_traits::FromPrimitive;

pub type Coord = i32;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Pos {
    pub x: Coord,
    pub y: Coord,
}

#[derive(Copy, Clone)]
pub enum ShiftDir {
    Left,
    Right,
}

/// Number of clockwise rotations
#[derive(Copy, Clone, PartialEq, Debug, FromPrimitive)]
pub enum Rotations {
    Zero,
    One,
    Two,
    Three,
}

#[derive(Copy, Clone)]
pub enum RotateDir {
    CW,
    CCW,
}

pub fn p<T: Into<Coord>>(x: T, y: T) -> Pos {
    Pos::new(x, y)
}

impl Pos {
    pub fn new<T: Into<Coord>>(x: T, y: T) -> Pos {
        Pos {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, other: Pos) -> Pos {
        Pos::new(self.x + other.x, self.y + other.y)
    }
}

impl Add<ShiftDir> for Pos {
    type Output = Pos;
    fn add(self, other: ShiftDir) -> Pos {
        Pos::new(
            self.x
                + match other {
                    ShiftDir::Left => -1,
                    ShiftDir::Right => 1,
                },
            self.y,
        )
    }
}

impl Add<RotateDir> for Rotations {
    type Output = Self;
    fn add(self, other: RotateDir) -> Self {
        let delta = match other {
            RotateDir::CW => 1,
            RotateDir::CCW => 3,
        };

        Rotations::from_i32((self as i32 + delta) % 4).expect("Unexpected rotation count")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_pos() {
        assert_eq!(p(1, 2) + p(3, 4), p(4, 6));
    }

    #[test]
    fn shift_pos() {
        assert_eq!(p(4, 8) + ShiftDir::Left, p(3, 8));
        assert_eq!(p(4, 8) + ShiftDir::Right, p(5, 8));
    }

    #[test]
    fn add_rotations() {
        assert_eq!(Rotations::Two, Rotations::One + RotateDir::CW);
        assert_eq!(Rotations::Three, Rotations::Zero + RotateDir::CCW);
        assert_eq!(Rotations::Zero, Rotations::Three + RotateDir::CW);
    }
}
