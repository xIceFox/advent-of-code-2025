use std::cmp::max;

pub struct Interval {
    pub start: i64,
    pub end: i64,
}

pub struct IntervalTree {
    start_node: Option<Box<IntervalNode>>,
}

impl IntervalTree {
    pub fn new_empty() -> Self {
        Self { start_node: None }
    }

    pub fn new(init_vec : &[(i64, i64)]) -> Self {
        let mut intervals: Vec<(i64, i64)> = init_vec
            .iter()
            .map(|(start, end)| {
                if start <= end {
                    (*start, *end)
                } else {
                    (*end, *start)
                }
            })
            .collect();

        intervals.sort_by_key(|(start, _)| *start);

        Self { start_node: Self::build_tree(&intervals) }
    }

    fn build_tree(intervals: &[(i64, i64)]) -> Option<Box<IntervalNode>> {
        if intervals.is_empty() {
            return None;
        }

        let mid = intervals.len() / 2;
        let (start, end) = intervals[mid];
        let mut node = IntervalNode::new_box(start, end, 1);
        node.left  = Self::build_tree(&intervals[..mid]);
        node.right = Self::build_tree(&intervals[mid+1..]);
        node.update_max();
        node.update_height();
        Some(node)
    }

    pub fn insert(&mut self, start: i64, end: i64) {
        let (start, end) = if start <= end { (start, end) } else { (end, start) };

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

    pub fn contains_including_borders(&self, x: i64) -> bool {
        self.contains(x, true, true)
    }

    pub fn contains(&self, x: i64, include_start: bool, include_end: bool) -> bool {
        if let Some(ref root) = self.start_node {
            root.contains(x, include_start, include_end)
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

struct IntervalNode {
    start: i64,
    end: i64,
    max_end: i64,
    height: u32,
    left: Option<Box<IntervalNode>>,
    right: Option<Box<IntervalNode>>,
}

impl IntervalNode {
    fn new_box(start: i64, end: i64, height: u32) -> Box<Self> {
        Box::new(Self {
            start,
            end,
            max_end: end,
            height,
            left: None,
            right: None,
        })
    }

    fn insert(mut self: Box<Self>, start: i64, end: i64) -> Box<IntervalNode> {
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
            return if start < self.left.as_ref().unwrap().start {
                // Left-Left case
                self.rotate_right()
            } else {
                // Left-Right case
                self.left = Some(self.left.take().unwrap().rotate_left());
                self.rotate_right()
            }
        } else if balance < -1 {
            // Right heavy
            return if start >= self.right.as_ref().unwrap().start {
                // Right-Right case
                self.rotate_left()
            } else {
                // Right-Left case
                self.right = Some(self.right.take().unwrap().rotate_right());
                self.rotate_left()
            }
        }

        self
    }

    fn contains(&self, x: i64, include_start: bool, include_end: bool) -> bool {
        let start_ok = if include_start { self.start <= x } else { self.start < x };
        let end_ok = if include_end { x <= self.end } else { x < self.end };
        let max_ok = if include_end { x <= self.max_end } else { x < self.max_end };

        if start_ok && end_ok {
            return true;
        }

        if let Some(ref left) = self.left
            && max_ok
            && left.contains(x, include_start, include_end)
        {
            return true;
        }

        if let Some(ref right) = self.right
            && max_ok
            && start_ok
            && right.contains(x, include_start, include_end)
        {
            return true;
        }

        false
    }

    fn merge(&self, list: &mut Vec<Interval>) {
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

    fn rotate_right(mut self: Box<Self>) -> Box<IntervalNode> {
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

    fn rotate_left(mut self: Box<Self>) -> Box<IntervalNode> {
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

    fn update_height(&mut self) {
        let left_height = self.left.as_ref().map_or(0, |n| n.height);
        let right_height = self.right.as_ref().map_or(0, |n| n.height);

        self.height = 1 + max(left_height, right_height);
    }

    fn update_max(&mut self) {
        let left_max = self.left.as_ref().map_or(self.end, |n| n.max_end);
        let right_max = self.right.as_ref().map_or(self.end, |n| n.max_end);

        self.max_end = max(self.end, max(left_max, right_max));
    }

    fn get_balance(&self) -> i32 {
        let left_height = self.left.as_ref().map_or(0, |n| n.height);
        let right_height = self.right.as_ref().map_or(0, |n| n.height);
        left_height as i32 - right_height as i32
    }

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