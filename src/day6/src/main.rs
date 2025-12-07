struct Problem {
    numbers: Vec<u64>,
    operation: Option<Operation>,
}

impl Problem {
    fn new() -> Self {
        Problem {
            numbers: vec![],
            operation: None,
        }
    }

    fn solve(&self) -> u64 {
        if self.operation.is_none() {
            panic!("No operation was added before calling solve")
        }

        let operation = self.operation.as_ref().unwrap();

        let initial_accumulator = match operation {
            Operation::Add => 0,
            Operation::Multiply => 1,
        };

        self.numbers.iter().fold(initial_accumulator, |acc, num| {
            operation.apply(acc, *num)
        })
    }

    fn add_number(&mut self, number: u64) {
        self.numbers.push(number);
    }

    fn add_operation(&mut self, op: Operation) {
        self.operation = Some(op);
    }
}

enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn parse_from_symbol(symbol: char) -> Option<Self> {
        match symbol {
            '+' => Some(Operation::Add),
            '*' => Some(Operation::Multiply),
            _ => None,
        }
    }

    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
        }
    }
}

fn uniform_line_length(grid: &[Vec<char>]) -> Option<usize> {
    if grid.is_empty() {
        return Some(0);
    }

    let first = grid.first().unwrap();

    if grid.iter().all(|row| row.len() == first.len()) {
        Some(first.len())
    } else {
        None
    }
}

fn main() {
    const PATH: &str = "src/day6/input.txt";
    let lines = core::read_lines(PATH).unwrap();

    let mut problems: Vec<Problem> = vec![];

    for line in lines {
        if line.is_err() {
            break;
        }

        let line_unwrapped = line.unwrap();

        let parts = line_unwrapped.split_whitespace();

        for (index, part) in parts.enumerate() {
            if index >= problems.len() {
                problems.push(Problem::new())
            }

            let problem = &mut problems[index];

            if part == "*" || part == "+" {
                let symbol = part.chars().next().unwrap();
                problem.add_operation(Operation::parse_from_symbol(symbol).expect("Invalid input"));
                continue;
            }

            let number = part.parse::<u64>().expect("Could not parse number!");
            problem.add_number(number);
        }
    }

    let result: u64 = problems
        .iter()
        .map(|problem| {
            problem.solve()
        })
        .sum();

    println!("Result: {}", result);

    // now part 2 ... uff

    let lines2 = core::read_lines(PATH).unwrap();

    let grid: Vec<Vec<char>> = lines2
        .map_while(Result::ok)
        .map(|x| x.chars().collect())
        .collect();

    let line_length = uniform_line_length(&grid).expect("Invalid input format!");

    let mut number_stack: Vec<u64> = vec![];
    let mut total = 0_u64;

    for x in (0..line_length).rev(){
        let mut multiplier = 1;
        let mut num = 0_u32;
        for y in (0..grid.len()-1).rev() {
            let digit_char = grid[y][x];
            if digit_char == ' '{
                continue;
            }

            let digit = digit_char.to_digit(10).expect("Invalid input!");
            num += multiplier * digit;
            multiplier *= 10;
        }

        if num > 0 {
            number_stack.push(num as u64);
        }

        let operation_char = grid[grid.len()-1][x];
        if operation_char == ' ' {
            continue;
        }

        let operation = Operation::parse_from_symbol(operation_char).expect("invalid input");

        let mut acc = match operation {
            Operation::Add => 0,
            Operation::Multiply => 1,
        };

        while let Some(number) = number_stack.pop(){
            acc = operation.apply(acc, number);
        }
        total += acc;
    }

    println!("Result Part2: {}", total);
}
