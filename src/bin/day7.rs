use aoc2025::utils;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;

static DOWN: (i32, i32) = (1, 0);
static RIGHT: (i32, i32) = (0, 1);
static LEFT: (i32, i32) = (0, -1);

fn part_1(grid: &Vec<Vec<char>>) -> u64 {
    let start: (i32, i32) = (0, grid[0].iter().position(|&ch| ch == 'S').unwrap() as i32);
    let n: i32 = grid.len() as i32;
    let m: i32 = grid[0].len() as i32;

    let mut res = 0;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    q.push_back(start);
    visited.insert(start);

    while !q.is_empty() {
        for _ in 0..q.len() {
            let (x, y) = q.pop_front().unwrap();
            let (dx, dy) = (x + DOWN.0, y + DOWN.1);
            if dx >= 0 && dx < n && dy >= 0 && dy < m && !visited.contains(&(dx, dy)) {
                if grid[dx as usize][dy as usize] == '.' {
                    q.push_back((dx, dy));
                    visited.insert((dx, dy));
                } else if grid[dx as usize][dy as usize] == '^' {
                    // have to split here
                    let (dx_left, dy_left) = (dx + LEFT.0, dy + LEFT.1);
                    let (dx_right, dy_right) = (dx + RIGHT.0, dy + RIGHT.1);
                    res += 1;
                    if dx_left >= 0
                        && dx_left < n
                        && dy_left >= 0
                        && dy_left < m
                        && !visited.contains(&(dx_left, dy_left))
                    {
                        q.push_back((dx_left, dy_left));
                        visited.insert((dx_left, dy_left));
                    }
                    if dx_right >= 0
                        && dx_right < n
                        && dy_right >= 0
                        && dy_right < m
                        && !visited.contains(&(dx_right, dy_right))
                    {
                        q.push_back((dx_right, dy_right));
                        visited.insert((dx_right, dy_right));
                    }
                }
            }
        }
    }
    res
}


// this is basically find all unique paths in grid with obstacles
fn part_2(grid: &Vec<Vec<char>>) -> u64 {
    let start: (i32, i32) = (0, grid[0].iter().position(|&ch| ch == 'S').unwrap() as i32);
    let n: i32 = grid.len() as i32;
    let m: i32 = grid[0].len() as i32;

    let mut memo: HashMap<(i32, i32), u64> = HashMap::new();

    fn recurse(
        x: i32,
        y: i32,
        n: i32,
        m: i32,
        grid: &Vec<Vec<char>>,
        memo: &mut HashMap<(i32, i32), u64>,
    ) -> u64 {
        // Check memo first
        if let Some(&result) = memo.get(&(x, y)) {
            return result;
        }

        let mut dx = x;
        let dy = y;
        // keep going down while we can
        while dx + DOWN.0 < n && grid[(dx + DOWN.0) as usize][dy as usize] == '.' {
            dx += DOWN.0;
        }

        // check if we've reached the bottom or gone past it
        if dx + DOWN.0 >= n {
            return 1 as u64;
        }
        
        // handle case where we hit something invalid
        if grid[(dx + DOWN.0) as usize][dy as usize] != '^' {
            return 0; // hit a wall or invalid position
        }

        // at this point dx is right above the splitter "^"
        dx += DOWN.0 * 2; // move to past splitter pos 
        let result = recurse(dx, dy + LEFT.1, n, m, grid, memo)
            + recurse(dx, dy + RIGHT.1, n, m, grid, memo);

        // Store in memo before returning
        memo.insert((x, y), result);
        result
    }

    recurse(start.0, start.1, n, m, &grid, &mut memo)
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let input: Vec<String> = utils::read_lines(7);
    // convert to vec<vec<char>> to iterate easily
    let grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let part1 = part_1(&grid.clone());
    let part2 = part_2(&grid.clone());
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
