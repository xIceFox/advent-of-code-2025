use crate::dial_mod::direction::Direction;
use crate::dial_mod::instruction::Instruction;

pub(crate) struct Dial {
    pub(crate) position: u8,
    pub(crate) zero_hits: u32,
    pub(crate) zero_wraps: u32,
}

impl Dial {
    pub(crate) fn new(init_pos: u8) -> Self {
        Dial {
            position: init_pos,
            zero_hits: 0,
            zero_wraps: 0,
        }
    }

    pub(crate) fn turn(&mut self, instruction: &Instruction) {
        let steps = match instruction.direction {
            Direction::Left => -(instruction.steps as i32),
            Direction::Right => instruction.steps as i32,
        };
        
        let pos_start = self.position as i32;
        let pos_after = pos_start + steps;

        self.position = pos_after.rem_euclid(100) as u8;

        if self.position == 0 {
            self.zero_hits += 1;
        }

        // always add full rotations
        let full_rotations = instruction.steps / 100;
        let mut remainder = instruction.steps as i32 % 100;

        if instruction.direction == Direction::Left {
            remainder *= -1;
        }

        self.zero_wraps += full_rotations;

        if pos_start != 0 && (pos_start + remainder <= 0 || pos_start + remainder >= 100) {
            self.zero_wraps += 1;
        }
    }
}