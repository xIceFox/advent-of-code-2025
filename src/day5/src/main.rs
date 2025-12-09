use core::datastructures::interval_tree::IntervalTree;

fn main() {
    const PATH: &str = "src/day5/input.txt";
    let mut lines = core::read_lines(PATH).unwrap();

    let mut fresh: u32 = 0;

    let mut intervals: Vec<(i64, i64)> = vec![];

    while let Some(line) = lines.next() {
        let unwrapped = line.unwrap();

        if unwrapped.is_empty() {
            break;
        }

        let split: Vec<&str> = unwrapped.split("-").collect();
        let start = split[0].parse::<i64>().unwrap();
        let end = split[1].parse::<i64>().unwrap();

        intervals.push((start, end));
    }

    let tree = IntervalTree::new(&intervals);

    while let Some(line) = lines.next() {
        let unwrapped = line.unwrap();

        let number = unwrapped.parse::<i64>().unwrap();
        if tree.contains_including_borders(number) {
            fresh += 1;
        }
    }

    println!("Fresh: {}", fresh);

    let items: i64 = tree.merge().iter().map(|x| x.end - x.start + 1).sum();

    println!("Total unique fresh items {}", items);
}
