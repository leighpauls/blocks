use std::ops::Add;

pub type Coord = i32;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Pos {
    pub x: Coord,
    pub y: Coord,
}

#[derive(Copy, Clone)]
pub enum ShiftDir {
    Left,
    Right,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(Pos::new(1, 2) + Pos::new(3, 4), Pos::new(4, 6));
    }

    #[test]
    fn shift() {
        assert_eq!(Pos::new(4, 8) + ShiftDir::Left, Pos::new(3, 8));
        assert_eq!(Pos::new(4, 8) + ShiftDir::Right, Pos::new(5, 8));
    }
}
