use core::math::point3d::Point3D;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};

struct UnionFind {
    parent: Vec<usize>,
    group_count: usize
}

impl UnionFind{
    pub fn new(size: usize) -> Self {
        let mut parent = Vec::with_capacity(size);

        for i in 0..size {
            parent.push(i);
        }

        Self { parent , group_count: size}
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize){
        let rootx = self.find(x);
        let rooty = self.find(y);

        if rootx != rooty {
            self.parent[rooty] = rootx;
            self.group_count -= 1;
        }
    }
}

#[derive(Clone)]
struct DistanceItem {
    distance: f64,
    index_first: usize,
    index_second: usize
}

impl Eq for DistanceItem {}

impl PartialEq<Self> for DistanceItem {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl PartialOrd<Self> for DistanceItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DistanceItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.partial_cmp(&other.distance).unwrap()
    }
}

fn main() {
    const PATH: &str = "src/day8/input.txt";
    let lines = core::read_lines(PATH).unwrap();

    let points = lines
        .map_while(Result::ok)
        .map(|line| Point3D::parse(&line))
        .collect::<Result<Vec<Point3D>, String>>()
        .unwrap();

    let mut heap: BinaryHeap<Reverse<DistanceItem>> = BinaryHeap::new();
    for idx_outer in 0..points.len() {
        for idx_inner in idx_outer+1..points.len() {
            let distance_item = DistanceItem {
                distance: points[idx_outer].distance(&points[idx_inner]),
                index_first: idx_outer,
                index_second: idx_inner
            };
            heap.push(Reverse(distance_item))
        }
    }

    let mut groups: UnionFind = UnionFind::new(points.len());

    for _ in 0..1000{
        let distance_item = heap.pop().unwrap().0;

        groups.union(distance_item.index_first, distance_item.index_second);
    }

    let mut sizes: HashMap<usize, usize> = HashMap::new();
    for i in 0..points.len() {
        let root = groups.find(i);
        *sizes.entry(root).or_insert(0) += 1;
    }

    let mut group_sizes: Vec<usize> = sizes.values().cloned().collect();
    group_sizes.sort_unstable_by(|a, b| b.cmp(a));

    let answer: usize = group_sizes.iter().take(3).product();
    println!("Answer Part 1: {}", answer);

    loop {
        let distance_item = heap.pop().unwrap().0;

        groups.union(distance_item.index_first, distance_item.index_second);

        if groups.group_count <= 1 {
            let point1 = &points[distance_item.index_first];
            let point2 = &points[distance_item.index_second];
            println!("Answer Part 2: {}", point1.x * point2.x);
            break;
        }
    }
}
