use crate::controlled::{ControlledBlocks, DropResult};
use crate::field::{Field, PlayingFieldRenderBlocksInstructions};
use crate::position::{RotateDir, ShiftDir};
use crate::shapes::Shape;
use crate::time::GameClock;

pub struct GameState {
    field: Field,
    controlled_blocks: ControlledBlocks,
    clock: GameClock,
}

pub struct RenderInfo<'a> {
    pub playing_field: PlayingFieldRenderBlocksInstructions<'a>,
}

impl GameState {
    pub fn new() -> GameState {
        let clock = GameClock::new();
        let now = clock.now();
        GameState {
            field: Field::new(),
            controlled_blocks: ControlledBlocks::new(now, Shape::random()),
            clock: clock,
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

    pub fn render_info(&self) -> RenderInfo {
        RenderInfo {
            playing_field: PlayingFieldRenderBlocksInstructions::new(
                &self.field,
                self.controlled_blocks.minos(),
                self.controlled_blocks.ghost_minos(&self.field),
            ),
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

        // Replace the stopped blocks with new ones
        self.controlled_blocks = ControlledBlocks::new(self.clock.now(), Shape::random());
    }
}
