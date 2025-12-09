use crate::dial_mod::direction::Direction;

pub(crate) struct Instruction {
    pub(crate) direction: Direction,
    pub(crate) steps: u32,
}

impl Instruction {
    pub(crate) fn parse(instruction_str: &str) -> Option<Instruction> {
        if instruction_str.len() < 2 {
            return None;
        }

        let direction = match &instruction_str[0..1] {
            "R" => Direction::Right,
            "L" => Direction::Left,
            _ => return None,
        };

        let steps = instruction_str[1..].parse::<u32>().ok()?;

        Some(Instruction { direction, steps })
    }
}
