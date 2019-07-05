use std::ops::Add;

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

pub type RelativePoses = [Pos; 4];

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

impl Add<RelativePoses> for Pos {
    type Output = RelativePoses;
    fn add(self, other: RelativePoses) -> RelativePoses {
        [
            other[0] + self,
            other[1] + self,
            other[2] + self,
            other[3] + self,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(p(1, 2) + p(3, 4), p(4, 6));
    }

    #[test]
    fn shift() {
        assert_eq!(p(4, 8) + ShiftDir::Left, p(3, 8));
        assert_eq!(p(4, 8) + ShiftDir::Right, p(5, 8));
    }

    #[test]
    fn add_relative_poses() {
        let start: RelativePoses = [p(0, 0), p(1, 1), p(2, 2), p(3, 3)];
        let expected: RelativePoses = [p(1, 1), p(2, 2), p(3, 3), p(4, 4)];

        assert_eq!(expected, p(1, 1) + start);
    }
}
