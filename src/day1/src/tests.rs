#[cfg(test)]
mod tests {
    use crate::dial_mod::dial::Dial;
    use crate::dial_mod::direction::Direction;
    use crate::dial_mod::instruction::Instruction;

    #[test]
    fn test_l75_r20() {
        let mut dial = Dial::new(50);
        let instructions = [
            Instruction {
                direction: Direction::Left,
                steps: 75,
            },
            Instruction {
                direction: Direction::Right,
                steps: 20,
            },
        ];
        for instr in instructions {
            dial.turn(&instr);
        }
        assert_eq!(dial.zero_wraps, 1);
    }

    #[test]
    fn test_r75_l20() {
        let mut dial = Dial::new(50);
        let instructions = [
            Instruction {
                direction: Direction::Right,
                steps: 75,
            },
            Instruction {
                direction: Direction::Left,
                steps: 20,
            },
        ];
        for instr in instructions {
            dial.turn(&instr);
        }
        assert_eq!(dial.zero_wraps, 1);
    }

    #[test]
    fn test_l50_r50() {
        let mut dial = Dial::new(50);
        let instructions = [
            Instruction {
                direction: Direction::Left,
                steps: 50,
            },
            Instruction {
                direction: Direction::Right,
                steps: 50,
            },
        ];
        for instr in instructions {
            dial.turn(&instr);
        }
        assert_eq!(dial.zero_wraps, 1);
    }

    #[test]
    fn test_l50_l50() {
        let mut dial = Dial::new(50);
        let instructions = [
            Instruction {
                direction: Direction::Left,
                steps: 50,
            },
            Instruction {
                direction: Direction::Left,
                steps: 50,
            },
        ];
        for instr in instructions {
            dial.turn(&instr);
        }
        assert_eq!(dial.zero_wraps, 1);
    }

    #[test]
    fn test_r50_r50() {
        let mut dial = Dial::new(50);
        let instructions = [
            Instruction {
                direction: Direction::Right,
                steps: 50,
            },
            Instruction {
                direction: Direction::Right,
                steps: 50,
            },
        ];
        for instr in instructions {
            dial.turn(&instr);
        }
        assert_eq!(dial.zero_wraps, 1);
    }

    #[test]
    fn test_r50_l50() {
        let mut dial = Dial::new(50);
        let instructions = [
            Instruction {
                direction: Direction::Right,
                steps: 50,
            },
            Instruction {
                direction: Direction::Left,
                steps: 50,
            },
        ];
        for instr in instructions {
            dial.turn(&instr);
        }
        assert_eq!(dial.zero_wraps, 1);
    }

    // -------------------
    // Double-cross tests
    // -------------------

    #[test]
    fn test_l200() {
        let mut dial = Dial::new(50);
        let instructions = [Instruction {
            direction: Direction::Left,
            steps: 200,
        }];
        for instr in instructions {
            dial.turn(&instr);
        }
        assert_eq!(dial.zero_wraps, 2);
    }

    #[test]
    fn test_r200() {
        let mut dial = Dial::new(50);
        let instructions = [Instruction {
            direction: Direction::Right,
            steps: 200,
        }];
        for instr in instructions {
            dial.turn(&instr);
        }
        assert_eq!(dial.zero_wraps, 2);
    }

    #[test]
    fn test_l150_l50() {
        let mut dial = Dial::new(50);
        let instructions = [
            Instruction {
                direction: Direction::Left,
                steps: 150,
            },
            Instruction {
                direction: Direction::Left,
                steps: 50,
            },
        ];
        for instr in instructions {
            dial.turn(&instr);
        }
        assert_eq!(dial.zero_wraps, 2);
    }

    #[test]
    fn test_l150_r50() {
        let mut dial = Dial::new(50);
        let instructions = [
            Instruction {
                direction: Direction::Left,
                steps: 150,
            },
            Instruction {
                direction: Direction::Right,
                steps: 50,
            },
        ];
        for instr in instructions {
            dial.turn(&instr);
        }
        assert_eq!(dial.zero_wraps, 2);
    }

    #[test]
    fn test_r150_l50() {
        let mut dial = Dial::new(50);
        let instructions = [
            Instruction {
                direction: Direction::Right,
                steps: 150,
            },
            Instruction {
                direction: Direction::Left,
                steps: 50,
            },
        ];
        for instr in instructions {
            dial.turn(&instr);
        }
        assert_eq!(dial.zero_wraps, 2);
    }

    #[test]
    fn test_r150_r50() {
        let mut dial = Dial::new(50);
        let instructions = [
            Instruction {
                direction: Direction::Right,
                steps: 150,
            },
            Instruction {
                direction: Direction::Right,
                steps: 50,
            },
        ];
        for instr in instructions {
            dial.turn(&instr);
        }
        assert_eq!(dial.zero_wraps, 2);
    }

    #[test]
    fn test_r50_r100() {
        let mut dial = Dial::new(50);
        let instructions = [
            Instruction {
                direction: Direction::Right,
                steps: 50,
            },
            Instruction {
                direction: Direction::Right,
                steps: 100,
            },
        ];
        for instr in instructions {
            dial.turn(&instr);
        }
        assert_eq!(dial.zero_wraps, 2);
    }

    #[test]
    fn test_r75_r50() {
        let mut dial = Dial::new(50);
        let instructions = [
            Instruction {
                direction: Direction::Right,
                steps: 50,
            },
            Instruction {
                direction: Direction::Right,
                steps: 100,
            },
        ];
        for instr in instructions {
            dial.turn(&instr);
        }
        assert_eq!(dial.zero_wraps, 2);
    }

    #[test]
    fn test_r100() {
        let mut dial = Dial::new(0);
        let instructions = [Instruction {
            direction: Direction::Right,
            steps: 200,
        }];
        for instr in instructions {
            dial.turn(&instr);
        }
        assert_eq!(dial.zero_wraps, 2);
    }

    #[test]
    fn test_l100() {
        let mut dial = Dial::new(0);
        let instructions = [Instruction {
            direction: Direction::Left,
            steps: 200,
        }];
        for instr in instructions {
            dial.turn(&instr);
        }
        assert_eq!(dial.zero_wraps, 2);
    }

    #[test]
    fn test_r50_r1() {
        let mut dial = Dial::new(50);
        let instructions = [
            Instruction {
                direction: Direction::Right,
                steps: 50,
            },
            Instruction {
                direction: Direction::Right,
                steps: 1,
            },
        ];
        for instr in instructions {
            dial.turn(&instr);
        }
        assert_eq!(dial.zero_wraps, 1);
    }

    #[test]
    fn test_r50() {
        let mut dial = Dial::new(50);
        let instructions = [Instruction {
            direction: Direction::Right,
            steps: 50,
        }];
        for instr in instructions {
            dial.turn(&instr);
        }
        assert_eq!(dial.zero_wraps, 1);
    }

    #[test]
    fn test_l50_l1() {
        let mut dial = Dial::new(50);
        let instructions = [
            Instruction {
                direction: Direction::Left,
                steps: 50,
            },
            Instruction {
                direction: Direction::Left,
                steps: 1,
            },
        ];
        for instr in instructions {
            dial.turn(&instr);
        }
        assert_eq!(dial.zero_wraps, 1);
    }
}
