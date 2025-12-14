use aoc2025::utils;
// use std::env;

fn vec_to_tuples(input: &Vec<String>) -> Vec<(i64, i64)> {
    input
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            (
                parts[0].parse::<i64>().unwrap(),
                parts[1].parse::<i64>().unwrap(),
            )
        })
        .collect()
}

fn part_1(input: &Vec<(i64, i64)>) {
    let mut max_area = 0;
    let n = input.len();
    for i in 0..n {
        for j in i + 1..n {
            let (a, b) = input[i];
            let (c, d) = input[j];
            let area = (a - c + 1).abs() * (b - d + 1).abs();
            max_area = max_area.max(area);
        }
    }
    println!("Part 1: {}", max_area);
}

fn part_2(red_tiles: &Vec<(i64, i64)>) {
    let n = red_tiles.len();

    fn intersects_rectangle_interior(
        a: (i64, i64),
        b: (i64, i64),
        rect_x1: i64,
        rect_x2: i64,
        rect_y1: i64,
        rect_y2: i64,
    ) -> bool {
        // coords of edge formed by red tiles a & b
        let (x1, y1) = a; 
        let (x2, y2) = b;

        if x1 == x2 {
            // Vertical edge
            let edge_x = x1;
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);

            (edge_x > rect_x1 && edge_x < rect_x2) && (min_y.max(rect_y1) < max_y.min(rect_y2))
        } else {
            // Horizontal edge
            let edge_y = y1;
            let min_x = x1.min(x2);
            let max_x = x1.max(x2);

            (edge_y > rect_y1 && edge_y < rect_y2) && (min_x.max(rect_x1) < max_x.min(rect_x2))
        }
    }
    fn is_valid_rectangle(
        rect_x1: i64,
        rect_x2: i64,
        rect_y1: i64,
        rect_y2: i64,
        red_tiles: &Vec<(i64, i64)>,
    ) -> bool {
        let n = red_tiles.len();

        // Check if any polygon edge intersects the rectangle's interior
        for i in 0..n {
            let a = red_tiles[i];
            let b = red_tiles[(i + 1) % n];

            if intersects_rectangle_interior(
                a, b, rect_x1, rect_x2, rect_y1, rect_y2,
            ) {
                return false;
            }
        }

        true
    }

    let mut max_area = 0;

    for i in 0..n {
        for j in i + 1..n {
            let (a, b) = red_tiles[i];
            let (c, d) = red_tiles[j];

            // Skip if same row or same column (not opposite corners)
            if a == c || b == d {
                continue;
            }

            let x1 = a.min(c);
            let x2 = a.max(c);
            let y1 = b.min(d);
            let y2 = b.max(d);

            if is_valid_rectangle(x1, x2, y1, y2, red_tiles) {
                let area = (x2 - x1 + 1) * (y2 - y1 + 1);
                max_area = max_area.max(area);
            }
        }
    }

    println!("Part 2: {}", max_area);
}
fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    let input: Vec<String> = utils::read_lines(9);
    let input_tuples = vec_to_tuples(&input);
    part_1(&input_tuples);
    part_2(&input_tuples);
}
