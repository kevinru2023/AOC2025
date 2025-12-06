use aoc2025::utils;
use std::collections::HashSet;
use std::{env, vec};

/// Sorts ingredient ranges by their start values in ascending order.
/// Each range is expected to be in the format "start-end".
/// Also instead of using the current string format, we will
/// convert to tuples of (start, end) : (u64, u64)
fn sort_ingredient_ranges(ingredient_ranges: Vec<String>) -> Vec<(u64, u64)> {
    let mut parsed_ranges: Vec<(u64, u64)> = ingredient_ranges
        .iter()
        .map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            let start_num = start.parse::<u64>().unwrap();
            let end_num = end.parse::<u64>().unwrap();
            (start_num, end_num)
        })
        .collect();

    parsed_ranges.sort_by(|a, b| a.0.cmp(&b.0));
    parsed_ranges
}

/// Counts how many available ingredients fall within the given ingredient ranges.
/// Each ingredient is only counted once, even if it falls within multiple ranges.
fn part_1(ingredient_ranges: Vec<String>, avaliable: Vec<String>) -> u32 {
    let mut res = 0;
    let mut checked_ingredients: HashSet<u64> = HashSet::new();

    // check only the numbers that exist
    for ingredient_range in ingredient_ranges {
        let (start, end) = ingredient_range.split_once("-").unwrap();
        let start_num = start.parse::<u64>().unwrap();
        let end_num = end.parse::<u64>().unwrap();

        // numbers that are actually in our set
        for num_str in &avaliable {
            if let Ok(num) = num_str.parse::<u64>() {
                if num >= start_num && num <= end_num && !checked_ingredients.contains(&num) {
                    res += 1;
                    checked_ingredients.insert(num);
                }
            }
        }
    }
    res
}

// only need first half of input
fn part_2(ingredient_ranges: Vec<String>) -> u64 {
    let mut res = 0;

    // straight just merge intervals here now i.e. https://leetcode.com/problems/merge-intervals/
    let sorted_ranges: Vec<(u64, u64)> = sort_ingredient_ranges(ingredient_ranges.clone());
    let mut merged_ranges: Vec<(u64, u64)> = vec![sorted_ranges[0]]; // get first item

    for i in 1..sorted_ranges.len() {
        let last_idx = merged_ranges.len() - 1;
        let last = merged_ranges[last_idx];

        if sorted_ranges[i].0 <= last.1 {
            merged_ranges[last_idx].1 = last.1.max(sorted_ranges[i].1);
        } else {
            merged_ranges.push(sorted_ranges[i]);
        }
    }

    // now we simpliy count the items in each range
    for (start, end) in merged_ranges {
        res += end - start + 1;
    }

    res
}
fn main() {
    env::set_var("RUST_BACKTRACE", "1"); // for my debugging lol, culprit was line 82 (passed in wrong half of input)
    let input: Vec<String> = utils::read_lines(5);
    let new_line_pos = input.iter().position(|r| r.is_empty()).unwrap();
    let first_half = &input[0..new_line_pos];
    let second_half = &input[new_line_pos + 1..];
    let part_1_res = part_1(first_half.to_vec(), second_half.to_vec());
    let part_2_res = part_2(first_half.to_vec());
    println!("Part 1: {}", part_1_res);
    println!("Part 2: {}", part_2_res);
}
