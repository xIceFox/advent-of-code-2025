use std::cmp::max;

struct Interval {
    pub start: u64,
    pub end: u64,
}

struct IntervalTree {
    start_node: Option<Box<IntervalNode>>,
}

impl IntervalTree {
    pub fn new() -> Self {
        Self { start_node: None }
    }

    pub fn insert(&mut self, start: u64, end: u64) {
        if self.start_node.is_none() {
            self.start_node = Some(Box::new(IntervalNode {
                start,
                end,
                max_end: end,
                height: 0,
                left: None,
                right: None,
            }));
            return;
        }
        let node = self.start_node.take().unwrap().insert(start, end);
        self.start_node = Some(node);
    }

    pub fn contains(&self, x: u64) -> bool {
        if let Some(ref root) = self.start_node {
            root.contains(x)
        } else {
            false
        }
    }

    pub fn merge(&self) -> Vec<Interval> {
        let mut list: Vec<Interval> = vec![];
        if let Some(ref root) = self.start_node {
            root.merge(&mut list);
        }
        list
    }
}

struct IntervalNode {
    start: u64,
    end: u64,
    max_end: u64,
    height: u32,
    left: Option<Box<IntervalNode>>,
    right: Option<Box<IntervalNode>>,
}

impl IntervalNode {
    pub fn new_box(start: u64, end: u64, height: u32) -> Box<Self> {
        Box::new(Self {
            start,
            end,
            max_end: end,
            height,
            left: None,
            right: None,
        })
    }

    pub fn insert(mut self: Box<Self>, start: u64, end: u64) -> Box<IntervalNode> {
        let child = if start < self.start {
            &mut self.left
        } else {
            &mut self.right
        };

        if let Some(child_node) = child.take() {
            *child = Some(child_node.insert(start, end));
        } else {
            let new_node = IntervalNode::new_box(start, end, 1);
            *child = Some(new_node);
        };

        self.update_max();
        self.update_height();

        // After updating height in insert:
        let balance = self.get_balance();

        if balance > 1 {
            // Left heavy
            if start < self.left.as_ref().unwrap().start {
                // Left-Left case
                return self.rotate_right();
            } else {
                // Left-Right case
                self.left = Some(self.left.take().unwrap().rotate_left());
                return self.rotate_right();
            }
        } else if balance < -1 {
            // Right heavy
            if start >= self.right.as_ref().unwrap().start {
                // Right-Right case
                return self.rotate_left();
            } else {
                // Right-Left case
                self.right = Some(self.right.take().unwrap().rotate_right());
                return self.rotate_left();
            }
        }

        self
    }

    pub fn contains(&self, x: u64) -> bool {
        if self.start <= x && x <= self.end {
            return true;
        }

        if let Some(ref left) = self.left
            && left.max_end >= x
            && left.contains(x)
        {
            return true;
        }

        if let Some(ref right) = self.right
            && right.max_end >= x
            && right.contains(x)
        {
            return true;
        }

        false
    }

    pub fn merge(&self, list: &mut Vec<Interval>) {
        if let Some(left) = self.left.as_ref() {
            left.merge(list);
        }

        if !list.is_empty() {
            let last = list.last_mut().unwrap();
            if last.end + 1 >= self.start {
                last.end = max(last.end, self.end);
            } else {
                list.push(Interval {
                    start: self.start,
                    end: self.end,
                })
            }
        } else {
            list.push(Interval {
                start: self.start,
                end: self.end,
            })
        }

        if let Some(right) = self.right.as_ref() {
            right.merge(list);
        }
    }

    pub fn rotate_right(mut self: Box<Self>) -> Box<IntervalNode> {
        let mut new_root = self.left.take().unwrap();

        self.left = new_root.right.take();

        new_root.right = Some(self);

        // Update heights
        let right = new_root.right.as_mut().unwrap();
        right.update_height();
        right.update_max();

        new_root.update_height();
        new_root.update_max();

        new_root
    }

    pub fn rotate_left(mut self: Box<Self>) -> Box<IntervalNode> {
        let mut new_root = self.right.take().unwrap();

        self.right = new_root.left.take();

        new_root.left = Some(self);

        // Update heights
        let left = new_root.left.as_mut().unwrap();
        left.update_height();
        left.update_max();

        new_root.update_height();
        new_root.update_max();

        new_root
    }

    pub fn update_height(&mut self) {
        let left_height = self.left.as_ref().map_or(0, |n| n.height);
        let right_height = self.right.as_ref().map_or(0, |n| n.height);

        self.height = 1 + max(left_height, right_height);
    }

    pub fn update_max(&mut self) {
        let left_max = self.left.as_ref().map_or(self.end, |n| n.max_end);
        let right_max = self.right.as_ref().map_or(self.end, |n| n.max_end);

        self.max_end = max(self.end, max(left_max, right_max));
    }

    pub fn get_balance(&self) -> i32 {
        let left_height = self.left.as_ref().map_or(0, |n| n.height);
        let right_height = self.right.as_ref().map_or(0, |n| n.height);
        left_height as i32 - right_height as i32
    }
}

impl IntervalTree {
    pub fn print(&self) {
        if let Some(ref root) = self.start_node {
            println!(
                "Root: [{}-{}, max:{}, h:{}]",
                root.start, root.end, root.max_end, root.height
            );

            if let Some(ref left) = root.left {
                left.print_tree("".to_string(), true);
            }
            if let Some(ref right) = root.right {
                right.print_tree("".to_string(), false);
            }
        } else {
            println!("Empty tree");
        }
    }
}

impl IntervalNode {
    pub fn print_tree(&self, prefix: String, is_left: bool) {
        println!(
            "{}{}[{}-{}, max:{}, h:{}]",
            prefix,
            if is_left {
                "├──L: "
            } else {
                "└──R: "
            },
            self.start,
            self.end,
            self.max_end,
            self.height
        );

        let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });

        if let Some(ref left) = self.left {
            left.print_tree(new_prefix.clone(), true);
        }
        if let Some(ref right) = self.right {
            right.print_tree(new_prefix, false);
        }
    }
}

fn main() {
    const PATH: &str = "src/day5/input_example.txt";
    let lines = core::read_lines(PATH).unwrap();

    let mut tree = IntervalTree::new();
    let mut fresh: u32 = 0;

    let mut ranges: bool = true;
    for line in lines {
        if line.is_err() {
            break;
        }

        let unwrapped = line.unwrap();

        if unwrapped.is_empty() {
            ranges = false;
            tree.print();
            continue;
        }

        if ranges {
            let split: Vec<&str> = unwrapped.split("-").collect();
            let start = split[0].parse::<u64>().unwrap();
            let end = split[1].parse::<u64>().unwrap();

            tree.insert(start, end);
        }

        if !ranges {
            let number = unwrapped.parse::<u64>().unwrap();
            if tree.contains(number) {
                fresh += 1;
            }
        }
    }

    println!("Fresh: {}", fresh);

    let items: u64 = tree.merge().iter().map(|x| x.end - x.start + 1).sum();

    println!("Total unique fresh items {}", items);
}
