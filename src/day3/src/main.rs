struct Bank {
    slots: Vec<BatterySlot>,
    on_count: usize,
}

impl Bank {
    pub fn new(batteries: Vec<Battery>) -> Self {
        let slots = batteries
            .into_iter()
            .map(|battery| BatterySlot::new(false, battery))
            .collect();

        Self { slots, on_count: 0 }
    }

    pub fn turn_on(&mut self, turn_on_count: usize) {
        if self.on_count > 0 {
            panic!("Bank already on, turn off before turning on");
        }

        if turn_on_count > self.slots.len() {
            panic!("Wrong input data!");
        }

        let mut left_pointer: usize = 0;
        let mut right_pointer: usize = self.slots.len() - turn_on_count;

        while right_pointer < self.slots.len() {
            let mut max_joltage: u32 = 0;
            let mut max_idx = 0;
            for idx in left_pointer..=right_pointer {
                let joltage = self.slots[idx].battery.joltage;
                if joltage > max_joltage {
                    max_joltage = joltage;
                    max_idx = idx;
                }
            }

            self.slots[max_idx].turn_on();
            self.on_count += 1;
            left_pointer = max_idx + 1;
            right_pointer += 1;
        }
    }

    fn calculate_joltage(&self) -> u64 {
        self.slots
            .iter()
            .filter(|slot| slot.on)
            .enumerate()
            .fold(0, |acc, (idx, slot)| {
                let multiplier = 10_u64.pow((self.on_count - idx - 1) as u32);
                acc + (slot.battery.joltage as u64 * multiplier)
            })
    }
}

struct BatterySlot {
    on: bool,
    battery: Battery,
}

impl BatterySlot {
    pub fn new(on: bool, battery: Battery) -> Self {
        Self { on, battery }
    }

    pub fn turn_on(&mut self) {
        self.on = true;
    }
}

struct Battery {
    joltage: u32,
}

impl Battery {
    pub fn new(joltage: u32) -> Self {
        Battery { joltage }
    }
}

fn run(file_path: &str, turn_on_count: usize) -> u64 {
    let lines = core::read_lines(file_path).unwrap();

    let mut banks: Vec<Bank> = lines
        .map_while(Result::ok)
        .map(|x| {
            x.chars()
                .filter_map(|x| x.to_digit(10))
                .map(Battery::new)
                .collect()
        })
        .map(Bank::new)
        .collect();

    banks.iter_mut().for_each(|x| x.turn_on(turn_on_count));

    banks
        .iter()
        .fold(0, |acc, bank| acc + bank.calculate_joltage())
}

fn main() {
    const PATH: &str = "src/day3/input.txt";
    let total_joltage = run(PATH, 12);
    println!("{}", total_joltage)
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn test_example_part1(){
        const PATH: &str = "input_example.txt";
        let total_joltage = run(PATH, 2);
        assert_eq!(total_joltage, 357);
    }

    #[test]
    fn test_part1() {
        const PATH: &str = "input.txt";
        let total_joltage = run(PATH, 2);
        assert_eq!(total_joltage, 17_301);
    }

    #[test]
    fn test_example_part2(){
        const PATH: &str = "input_example.txt";
        let total_joltage = run(PATH, 12);
        assert_eq!(total_joltage, 3121910778619);
    }

    #[test]
    fn test_part2() {
        const PATH: &str = "input.txt";
        let total_joltage = run(PATH, 12);
        assert_eq!(total_joltage, 172_162_399_742_349);
    }
}
