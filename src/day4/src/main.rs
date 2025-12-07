struct Grid {
    grid: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(grid: Vec<Vec<bool>>) -> Self {
        let width = grid[0].len();
        let height = grid.len();
        Self {
            grid,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.grid[y][x]
    }

    fn remove(&mut self, x: usize, y: usize) {
        self.grid[y][x] = false;
    }

    fn get_adjacent(&self, x: usize, y: usize) -> Vec<bool> {
        [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ]
        .iter()
        .filter_map(|(dx, dy)| {
            let nx = x.checked_add_signed(*dx)?;
            let ny = y.checked_add_signed(*dy)?;
            if nx < self.width && ny < self.height {
                Some((nx, ny))
            } else {
                None
            }
        })
        .map(|(x, y)| self.get(x, y))
        .collect()
    }

    fn is_accessible(&self, x: usize, y: usize) -> bool {
        self.get_adjacent(x, y).into_iter().filter(|&x| x).count() < 4
    }

    fn get_accessible_count(&self) -> usize {
        let mut accessible_count: usize = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let is_roll_of_paper = self.get(x, y);
                if is_roll_of_paper {
                    if self.is_accessible(x, y) {
                        print!("x");
                        accessible_count += 1;
                    } else {
                        print!("@");
                    }
                } else {
                    print!(".")
                }
            }
            println!()
        }
        accessible_count
    }

    fn remove_accessible(&mut self) -> usize {
        let mut removed_count: usize = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let is_roll_of_paper = self.get(x, y);
                if is_roll_of_paper {
                    if self.is_accessible(x, y) {
                        print!("x");
                        removed_count += 1;
                        self.remove(x,y);
                    } else {
                        print!("@");
                    }
                } else {
                    print!(".")
                }
            }
            println!()
        }
        removed_count
    }

    fn remove_all(&mut self) -> usize{
        let mut total_removed = 0;
        loop{
            let removed_count = self.remove_accessible();
            total_removed += removed_count;

            if removed_count == 0 {
                break;
            }
        }
        total_removed
    }
}

fn main() {
    const PATH: &str = "src/day4/input.txt";
    let lines = core::read_lines(PATH).unwrap();

    let mut grid: Grid = Grid::new(
        lines
            .map_while(Result::ok)
            .map(|x| {
                x.chars()
                    .map(|x| {
                        if x == '@' {
                            true
                        } else if x == '.' {
                            false
                        } else {
                            panic!("wrong input, cannot parse")
                        }
                    })
                    .collect::<Vec<bool>>()
            })
            .collect(),
    );

    println!("Accessible: {}", grid.get_accessible_count());
    println!("TotalRemovable: {}", grid.remove_all());
}
