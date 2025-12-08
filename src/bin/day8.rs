use aoc2025::utils;
use std::{collections::{BinaryHeap, HashSet}, env, fmt};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)] // fyi for future me use these or printing is gonna be debugging hell
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z) // same as above lol
    }
}

// rust typing is so fun
fn distance(p1: &Point, p2: &Point) -> i64 {
    let dx = (p1.x as i64 - p2.x as i64).pow(2);
    let dy = (p1.y as i64 - p2.y as i64).pow(2);
    let dz = (p1.z as i64 - p2.z as i64).pow(2);
    ((dx + dy + dz) as f64).sqrt().round() as i64
}

fn to_point_vec(input: &Vec<String>) -> Vec<Point> {
    input
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            // turbo fish on top
            let x = parts[0].parse::<u32>().unwrap();
            let y = parts[1].parse::<u32>().unwrap();
            let z = parts[2].parse::<u32>().unwrap();
            Point { x, y, z }
        })
        .collect()
}

// this func will take in a vector of points and
// return the 1000 closest pairs as edges (idx pairs)
// to allow make it easier to perform union-find operations
fn create_graph(points: &Vec<Point>) -> Vec<(usize, usize)> {
    let mut heap: BinaryHeap<(i64, (usize, usize))> = BinaryHeap::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let dist = distance(&points[i], &points[j]);
            heap.push((-dist, (i, j)));
        }
    }
    let mut edges = Vec::new();
    let mut count = 0;
    while count < 1000 && !heap.is_empty() {
        // NOTE: swap this to 1000 later
        let (_, (p1, p2)) = heap.pop().unwrap();
        edges.push((p1, p2));
        count += 1;
    }
    edges
}

// Part 1 problem:
// given a bunch of points in a 3D graph want to connect together the points that are closet to each other
// based of their straight line distance. Want to connect the 1000 closest pair of points together and want
// to multiply the size of the 3 largest islands together.
//
// Idea:
// So lets break this down into smaller problems
// 1. First let's find out the closest pair of points and put them in a min heap
// 2. Then let's find out the 1000 closest pair of points by just popping 1000 items of the heap
// 3. Finally let's find out the size of the 3 largest islands and multiply them together ??

fn part_1(edges: Vec<(usize, usize)>, num_points: usize) {
    let mut dsu = utils::DSU::new(num_points);

    // Connect all the edges
    for (u, v) in &edges {
        dsu.union(*u, *v);
    }

    // Find all unique component sizes
    let mut sizes: BinaryHeap<usize> = BinaryHeap::new();
    let mut seen_roots = HashSet::new();

    for i in 0..num_points {
        let root = dsu.find(i);
        if seen_roots.insert(root) {
            sizes.push(dsu.component_size(root));
        }
    }

    // Multiply the 3 largest component sizes
    let mut res: u64 = 1;
    for _ in 0..3 {
        if let Some(size) = sizes.pop() {
            res *= size as u64;
        }
    }
    println!("Part 1: {}", res);
}

// modified create_graph func 
fn create_graph_part2(points: &Vec<Point>) -> Vec<(usize, usize)> {
    let mut heap: BinaryHeap<(i64, (usize, usize))> = BinaryHeap::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let dist = distance(&points[i], &points[j]);
            // note for some reason docs mention use of reverse, but the python in me thinks it's cleaner to user negative values
            heap.push((-dist, (i, j))); 
        }
    }
    let mut edges = Vec::new();
    while !heap.is_empty() {
        let (_, (p1, p2)) = heap.pop().unwrap();
        edges.push((p1, p2));
    }
    edges
}

// Part 2 problem:
// Continue connecting the closest unconnected pairs of points together until they're all in the same component
// then return the last two x coordinates of the last two points multiplied
//
// Idea:
// This is way easier than the last one since most of the logic is already done, we just need to refine
// our graph a bit to instead just contain all edges in a sorted order and then we will use our built in
// lib function to see the size of our current component

fn part_2(edges: Vec<(usize, usize)>, num_points: usize, points: &Vec<Point>) {
    let mut uf = utils::DSU::new(num_points); // prob should have moved this to a dif crate
    let mut last_pair = (0, 0);

    for (p1, p2) in edges {
        // Check if already connected (same component)
        if !uf.connected(p1, p2) {
            uf.union(p1, p2);
            if uf.component_size(p1) == num_points {
                last_pair = (p1, p2);
                break;
            }
        }
    }
    let res: u64 = points[last_pair.0].x as u64 * points[last_pair.1].x as u64; // overflow was causing me a headache so we ball 
    println!("Part 2: {}", res);
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let input: Vec<String> = utils::read_lines(8);
    let points = to_point_vec(&input);
    // below is useful for debugging the points fyi
    // for (point, idx) in points.iter().enumerate() {
    //     println!("Point: {:?} | idx: {}", point, idx);
    // }
    let graph = create_graph(&points);
    part_1(graph, points.len());
    part_2(create_graph_part2(&points), points.len(), &points);
}
