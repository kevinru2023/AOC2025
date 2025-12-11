use aoc2025::utils;
use std::env;

// This problem trickles down to Given a list of numbers, find the minimum subset whose XOR equals the target.
// So first we are going to have to parse the input into a list of numbers and a target number
// so from an input like [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
// we will parse out the [.##.] as the target number (6)
// and repr each list of numbers as their sum of their binary repr
// so output from above input would be like
// ([1,5,2,3, 10, 12], 6)
fn parse_line(line: &String) -> (Vec<u32>, u32) {
    let parts = line.split(' ').collect::<Vec<&str>>();
    let n = parts.len();
    let mut numbers: Vec<u32> = Vec::new();
    let mut target: u32 = 0;
    let mut bits = 0;
    // parse the target number
    let mut num: u32 = 0;
    let binary_str = &parts[0][1..parts[0].len() - 1];
    bits = binary_str.len() as u32;
    for i in 0..binary_str.len() {
        if let Some(c) = binary_str.chars().nth(i) {
            if c == '#' {
                num |= 1 << (binary_str.len() - 1 - i);
            }
        }
    }
    target = num;
    num = 0; // reset num

    // parse the list of numbers
    for i in 1..n - 1 {
        let lst = &parts[i][1..parts[i].len() - 1]
            .split(',')
            .collect::<Vec<&str>>();
        for number_str in lst {
            if let Some(number) = number_str.parse::<u32>().ok() {
                num |= 1 << (bits - 1 - number);
            }
        }
        numbers.push(num);
        num = 0; // reset num
    }
    (numbers, target)
}

fn search(k: usize, subset: &mut Vec<u32>,numbers: &Vec<u32>,target: u32, min_size: &mut usize) {
    if k == numbers.len(){
        // process subset
        let mut xor_res: u32 = 0;
        for &num in subset.iter() {
            xor_res ^= num;
        }
        if xor_res == target {
            *min_size = (*min_size).min(subset.len());
        }
    } else {
        search(k + 1, subset, numbers, target, min_size);
        subset.push(numbers[k]);
        search(k + 1, subset, numbers, target, min_size);
        subset.pop();
    }
}

fn part_1(input: &Vec<String>) {
    let mut res: u32 = 0; 
    for line in input {
        let (numbers, target) = parse_line(line);
        let mut min_num: usize = usize::MAX;
        let mut subset: Vec<u32> = Vec::new(); 
        search(0, &mut subset, &numbers, target, &mut min_num);
        res += min_num as u32;
    }
    println!("Part 1: {}", res);
}

fn part_2() {}

fn main() {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }
    let input: Vec<String> = utils::read_lines(10);
    part_1(&input);
    part_2();
}
