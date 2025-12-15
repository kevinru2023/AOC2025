use std::collections::HashMap;

use aoc2025::utils;

fn get_shapes(input: &Vec<String>) -> Vec<Vec<String>> {
    let mut shapes: Vec<Vec<String>> = Vec::new();
    let m = input.iter().position(|s| s.contains("x")).unwrap_or(0);

    let mut tmp_vec: Vec<String> = Vec::new();

    for i in 0..m {
        if input[i].contains(":") {
            if !tmp_vec.is_empty() {
                shapes.push(tmp_vec.clone());
            }
            tmp_vec.clear();
        } else if input[i].is_empty() {
            if !tmp_vec.is_empty() {
                shapes.push(tmp_vec.clone());
                tmp_vec.clear();
            }
        } else {
            tmp_vec.push(input[i].clone());
        }
    }

    shapes
}

fn get_regions(input: &Vec<String>) -> Vec<String> {
    let mut regions: Vec<String> = Vec::new();
    let m = input.iter().position(|s| s.contains("x")).unwrap_or(0);
    for i in m..input.len() {
        regions.push(input[i].clone());
    }
    regions
}

fn part_1(shapes: &Vec<Vec<String>>, regions: &Vec<String>) {
    let mut shape_to_area: HashMap<usize, u32> = HashMap::new();
    let mut res = 0;
    for (idx, shape) in shapes.iter().enumerate() {
        let mut area = 0;
        for line in shape {
            for ch in line.chars() {
                if ch == '#' {
                    area += 1;
                }
            }
        }
        shape_to_area.insert(idx, area);
    }

    for region in regions {
        let parts: Vec<&str> = region.split_whitespace().collect();

        let dims = parts[0].trim_end_matches(':');
        let x_idx: usize = dims.find("x").unwrap_or(0);
        let l = dims[..x_idx].parse::<u32>().unwrap_or(0);
        let w = dims[x_idx + 1..].parse::<u32>().unwrap_or(0);

        let area = l * w;
        let mut total_area = 0;
        let mut presents = 0;
        
        for i in 1..parts.len() {
            if let Ok(amount) = parts[i].parse::<u32>() {
                let shape_area = shape_to_area.get(&(i - 1)).unwrap_or(&0);
                total_area += shape_area * amount;
                presents += amount;
            }
        }
        
        
        let max_presents_lower_bound = (w / 3) * (l / 3);
        println!("max_presents_lower_bound: {}", max_presents_lower_bound);
        if total_area <= area || presents <= max_presents_lower_bound {
            res += 1;
            continue;
        }
    }
    println!("Part 1: {}", res);
}

fn part_2() {}

fn main() {
    let input: Vec<String> = utils::read_lines(12);
    let shapes = get_shapes(&input);
    let regions = get_regions(&input);
    part_1(&shapes, &regions);
    part_2();
}
