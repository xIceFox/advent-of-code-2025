use std::cmp::PartialEq;
use std::collections::HashMap;
use std::process::ExitCode;

#[derive(Clone, Copy, PartialEq)]
pub enum Element {
    Start,
    Splitter,
    Beam,
    Empty,
}

impl Element {
    pub fn parse_from_char(chr: char) -> Option<Self> {
        match chr {
            'S' => Some(Self::Start),
            '^' => Some(Self::Splitter),
            '|' => Some(Self::Beam),
            '.' => Some(Self::Empty),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Start => 'S',
            Self::Splitter => '^',
            Self::Beam => '|',
            Self::Empty => '.',
        }
    }
}

pub struct Grid {
    data: HashMap<(i64, i64), Element>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(data: HashMap<(i64, i64), Element>) -> Self {
        let width = data.keys().map(|(x, _)| *x).max().unwrap_or(0) as usize + 1;
        let height = data.keys().map(|(_, y)| *y).max().unwrap_or(0) as usize + 1;
        Self {
            data,
            width,
            height,
        }
    }

    pub fn is_valid(&self) -> bool {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(&(x as i64, y as i64)).is_none() {
                    return false;
                }
            }
        }
        true
    }

    pub fn get(&self, coordinates: &(i64, i64)) -> Option<Element> {
        self.data.get(coordinates).copied()
    }

    pub fn change(&mut self, coordinates: &(i64, i64), element: Element) {
        if self.data.contains_key(coordinates) {
            self.data.insert(*coordinates, element);
        }
    }

    pub fn print(&self) {
        let mut visual = vec![vec![Element::Empty; self.width]; self.height];

        for (&(x, y), &elem) in &self.data {
            visual[y as usize][x as usize] = elem;
        }

        for row in visual {
            for elem in row {
                print!("{}", elem.to_char());
            }
            println!();
        }
    }
}

fn main() -> ExitCode {
    const PATH: &str = "src/day7/input.txt";
    let lines = core::read_lines(PATH).unwrap();

    let mut starts: Vec<(i64, i64)> = vec![];
    let mut grid: Grid = Grid::new(
        lines
            .map_while(Result::ok)
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, chr)| {
                        let element = Element::parse_from_char(chr).expect("Invalid input!");

                        if element == Element::Start {
                            starts.push((x as i64, y as i64));
                        }

                        ((x as i64, y as i64), element)
                    })
                    .collect::<Vec<((i64, i64), Element)>>()
            })
            .collect(),
    );

    if !grid.is_valid() {
        println!("Read-in grid is not valid, it is not rectangular!");
        return ExitCode::FAILURE;
    }

    if starts.is_empty() {
        println!("No start found, check input data and add 'S' symbol somewhere!");
        return ExitCode::FAILURE;
    }


    let mut beams: HashMap<(i64, i64), u64> = HashMap::new();
    let mut split_count: u32 = 0;
    let mut timeline_count = 0;

    for &x in &starts {
        beams.insert(x, 1);
    }

    while !beams.is_empty() {
        let mut next_beams: HashMap<(i64, i64), u64> = HashMap::new();

        for beam in beams {
            let coordinates = beam.0;

            let down_coordinates = (coordinates.0, coordinates.1 + 1);
            let (x, y) = down_coordinates;

            let elem = grid.get(&down_coordinates);

            if elem.is_none() {
                timeline_count += beam.1;
                continue;
            }

            let elem = elem.unwrap();

            match elem {
                Element::Empty => {
                    grid.change(&down_coordinates, Element::Beam);
                }
                Element::Splitter => {
                    let splitter_left_coordinates = (x - 1, y);
                    let splitter_right_coordinates = (x + 1, y);

                    next_beams
                        .entry(splitter_left_coordinates)
                        .and_modify(|entry| *entry += beam.1)
                        .or_insert(beam.1);

                    next_beams
                        .entry(splitter_right_coordinates)
                        .and_modify(|entry| *entry += beam.1)
                        .or_insert(beam.1);

                    grid.change(&splitter_left_coordinates, Element::Beam);
                    grid.change(&splitter_right_coordinates, Element::Beam);
                    split_count += 1;
                    continue
                }
                Element::Beam => {},
                _ => panic!("Hit wrong element, we are doomed!"),
            }

            next_beams
                .entry(down_coordinates)
                .and_modify(|entry| *entry += beam.1)
                .or_insert(beam.1);
        }
        beams = next_beams;
    }

    grid.print();

    println!("Split count: {}", split_count);
    println!("Timeline count: {}", timeline_count);

    ExitCode::SUCCESS
}
