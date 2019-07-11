use crate::controlled::{ControlledBlocks, DropResult};
use crate::field::{Field, PlayingFieldRenderBlocksInstructions};
use crate::position::{RotateDir, ShiftDir};
use crate::random_bag::RandomBag;
use crate::shapes::Shape;
use crate::time::GameClock;

pub struct GameState {
    field: Field,
    controlled_blocks: ControlledBlocks,
    clock: GameClock,
    random_bag: RandomBag,
    hold_piece: Option<Shape>,
    can_hold: bool,
}

pub struct RenderInfo<'a> {
    pub playing_field: PlayingFieldRenderBlocksInstructions<'a>,
    pub previews: Vec<Shape>,
    pub hold_piece: Option<Shape>,
}

impl GameState {
    pub fn new() -> GameState {
        let clock = GameClock::new();
        let now = clock.now();
        let mut rb = RandomBag::new();

        GameState {
            field: Field::new(),
            controlled_blocks: ControlledBlocks::new(now, rb.take_next()),
            clock: clock,
            random_bag: rb,
            hold_piece: None,
            can_hold: true,
        }
    }

    pub fn update(&mut self) {
        let drop_result = self
            .controlled_blocks
            .maybe_periodic_drop(&self.field, self.clock.now());
        self.handle_soft_drop(drop_result);
    }

    pub fn on_input_shift(&mut self, dir: ShiftDir) {
        self.controlled_blocks.shift(&self.field, dir);
    }

    pub fn on_input_soft_drop(&mut self) {
        let drop_result = self
            .controlled_blocks
            .manual_soft_drop(&self.field, self.clock.now());
        self.handle_soft_drop(drop_result)
    }

    pub fn on_input_rotate(&mut self, dir: RotateDir) {
        self.controlled_blocks.rotate(&self.field, dir);
    }

    pub fn on_input_hard_drop(&mut self) {
        self.controlled_blocks.hard_drop(&self.field);
        self.replace_controlled_piece();
    }

    pub fn on_input_hold_piece(&mut self) {
        if !self.can_hold {
            return;
        }
        let new_piece = match self.hold_piece {
            Some(shape) => shape,
            None => self.random_bag.take_next(),
        };
        self.hold_piece = Some(self.controlled_blocks.minos().shape());
        self.controlled_blocks = ControlledBlocks::new(self.clock.now(), new_piece);
        self.can_hold = false;
    }

    pub fn render_info(&self) -> RenderInfo {
        RenderInfo {
            playing_field: PlayingFieldRenderBlocksInstructions::new(
                &self.field,
                self.controlled_blocks.minos(),
                self.controlled_blocks.ghost_minos(&self.field),
            ),
            previews: self.random_bag.previews(),
            hold_piece: self.hold_piece,
        }
    }

    fn handle_soft_drop(&mut self, drop_result: DropResult) {
        if let DropResult::Stop = drop_result {
            self.replace_controlled_piece();
        }
    }

    fn replace_controlled_piece(&mut self) {
        self.controlled_blocks
            .minos()
            .apply_to_field(&mut self.field);

        self.field.remove_lines();
        self.can_hold = true;

        // Replace the stopped blocks with new ones
        self.controlled_blocks =
            ControlledBlocks::new(self.clock.now(), self.random_bag.take_next());
    }
}
