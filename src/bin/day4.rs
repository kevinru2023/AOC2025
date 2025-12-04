use aoc2025::utils;

static DIRS: [(i32, i32); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (-1, 1),
    (1, -1),
    (-1, -1),
];

fn part_1(grid: &[String]) -> u32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut res = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i].as_bytes()[j] == b'.' {
                continue;
            }
            let mut count = 0;
            for &(dx, dy) in &DIRS {
                let x = i as i32 + dx;
                let y = j as i32 + dy;
                if x >= 0
                    && x < n as i32
                    && y >= 0
                    && y < m as i32
                    && grid[x as usize].as_bytes()[y as usize] == b'@'
                {
                    count += 1;
                }
            }
            if count < 4 {
                res += 1;
            }
        }
    }
    res // fun rust return syntax lol
}

fn part_2(mut grid: Vec<String>) -> u32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut to_remove: Vec<(usize, usize)> = Vec::new();

    // Find all cells to remove in this iteration
    for i in 0..n {
        for j in 0..m {
            if grid[i].as_bytes()[j] == b'.' {
                continue;
            }

            let mut count = 0;
            for &(dx, dy) in &DIRS {
                let x = i as i32 + dx;
                let y = j as i32 + dy;
                if x >= 0
                    && x < n as i32
                    && y >= 0
                    && y < m as i32
                    && grid[x as usize].as_bytes()[y as usize] == b'@'
                {
                    count += 1;
                }
            }
            if count < 4 {
                to_remove.push((i, j)); // much smarter and don't have to deal with modifiying rust strings
            }
        }
    }

    // Base case: if no cells to remove
    if to_remove.is_empty() {
        return 0;
    }

    // Remove the cells by replacing them with '.'
    let res: u32 = to_remove.len() as u32;
    for (i, j) in to_remove {
        unsafe {
            // unsafe since rust can't guarantee memory safety, but since we know '.' is valid, it's bascially safe
            grid[i].as_bytes_mut()[j] = b'.';
        }
    }

    // Recursively call with the modified grid
    res + part_2(grid)
}

fn main() {
    let grid: Vec<String> = utils::read_lines(4);
    let result_1 = part_1(&grid);
    let result_2 = part_2(grid);
    println!("Part 1: {}", result_1);
    println!("Part 2: {}", result_2);
}
