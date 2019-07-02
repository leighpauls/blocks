use crate::field::Field;
use crate::position::{Pos, ShiftDir};
use std::time::{Duration, Instant};

pub struct ControlledBlocks {
    root_pos: Pos,
    relative_poses: [Pos; 4],
    next_drop_option: Option<Instant>,
}

const DROP_PERIOD: Duration = Duration::from_millis(1000);

pub enum DropResult {
    Continue,
    Stop,
}

impl ControlledBlocks {
    pub fn new() -> ControlledBlocks {
        ControlledBlocks {
            root_pos: Pos::new(0, 0),
            relative_poses: [
                Pos::new(0, 0),
                Pos::new(1, 0),
                Pos::new(2, 0),
                Pos::new(3, 0),
            ],
            next_drop_option: None,
        }
    }

    pub fn start(&mut self) {
        self.next_drop_option = Some(Instant::now() + DROP_PERIOD);
    }

    pub fn positions(&self) -> [Pos; 4] {
        let mut result = self.relative_poses;
        for pos in result.iter_mut() {
            *pos = *pos + self.root_pos;
        }
        result
    }

    pub fn is_controlled(&self, target: Pos) -> bool {
        for relative in self.relative_poses.iter() {
            if self.root_pos + *relative == target {
                return true;
            }
        }
        return false;
    }

    pub fn shift(&mut self, field: &Field, dir: ShiftDir) {
        // Don't move if it's not legal
        for pos in self.relative_poses.iter() {
            if !field.is_open(self.root_pos + *pos + dir) {
                return;
            }
        }
        self.root_pos = self.root_pos + dir;
    }

    pub fn maybe_periodic_drop(&mut self, field: &Field) -> DropResult {
        if *self.next_drop() > Instant::now() {
            return DropResult::Continue;
        }
        *self.next_drop() += DROP_PERIOD;
        self.soft_drop(field)
    }

    pub fn manual_soft_drop(&mut self, field: &Field) -> DropResult {
        *self.next_drop() = Instant::now() + DROP_PERIOD;
        self.soft_drop(field)
    }

    fn soft_drop(&mut self, field: &Field) -> DropResult {
        let delta = Pos::new(0, 1);
        for pos in self.relative_poses.iter() {
            if !field.is_open(self.root_pos + *pos + delta) {
                return DropResult::Stop;
            }
        }
        self.root_pos = self.root_pos + delta;
        DropResult::Continue
    }

    fn next_drop(&mut self) -> &mut Instant {
        self.next_drop_option
            .as_mut()
            .expect("Using ControlledBlocks before calling start()")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn controlled() {
        let mut b = ControlledBlocks::new();
        b.start();

        assert_eq!(true, b.is_controlled(Pos::new(0, 0)));
        assert_eq!(false, b.is_controlled(Pos::new(0, 1)));
    }
}
