use aoc2025::utils;
use std::env;
use z3::{ast::Int, Optimize};

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
    // parse the target number
    let mut num: u32 = 0;
    let binary_str = &parts[0][1..parts[0].len() - 1];
    let bits = binary_str.len() as u32;
    for i in 0..binary_str.len() {
        if let Some(c) = binary_str.chars().nth(i) {
            if c == '#' {
                num |= 1 << (binary_str.len() - 1 - i);
            }
        }
    }
    let target = num;
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

fn search(k: usize, subset: &mut Vec<u32>, numbers: &Vec<u32>, target: u32, min_size: &mut usize) {
    if k == numbers.len() {
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

// for part 2 we reduce the problem into the following, represent the list of numbers as vectors
// where the length of the vector is the number of elements in the last element in the input i.e. for input
// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
// {3, 5, 4, 7} would be the last input so we take our length as 4
// and for each element in the list, i.e. list[i], list[i] repr if 1 is located in the col of the vector
// i.e. for (3) we would get a vector like [0 0 0 1] and for (1,3) we would get a vector like [0 1 0 1]
// from here what we want to find out is the min amount of these vectors required to reach the last vector i.e. [3 5 4 7]
// we can do this by using z3 solver to have variables that repr the amount of a vector we have, i.e. we have an equation like
// x1[0 0 0 1] + x2[0 1 0 1] + ... + xn[...] (here n is the amount of vectors) = [3 5 4 7] and then solve for these variables

fn parse_line_into_vectors(line: &String) -> (Vec<Vec<u32>>, Vec<u32>) {
    let parts = line.split(' ').collect::<Vec<&str>>();
    let n = parts.len();
    let mut vecs: Vec<Vec<u32>> = Vec::new();
    // parse the target vector from the last element
    let target_str = &parts[n - 1][1..parts[n - 1].len() - 1];
    let target_vec: Vec<u32> = target_str
        .split(',')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    let bits = target_vec.len();
    // now parse the individual vectors
    for i in 1..n - 1 {
        let vec_str = &parts[i][1..parts[i].len() - 1];
        let mut vec = vec![0; bits];
        for num_str in vec_str.split(',') {
            if let Ok(idx) = num_str.parse::<usize>() {
                vec[idx] = 1;
            }
        }
        vecs.push(vec);
    }
    (vecs, target_vec)
}

// I am gonna keep it real, I needed heavy help from AI, with this part, z3 rust cargo crate is such a pain in the ass
// conceptually, really easy, rust implementation? Hell. 
fn compute_line(vecs: &Vec<Vec<u32>>, target_vec: &Vec<u32>) -> i64 {
    let opt = Optimize::new();

    let m = vecs.len();
    let n = target_vec.len();

    let x: Vec<Int> = (0..m).map(|i| Int::new_const(format!("x{}", i))).collect();

    for xi in &x {
        opt.assert(&xi.ge(&Int::from_i64(0)));
    }

    for r in 0..n {
        let mut terms = Vec::new();
        for j in 0..m {
            if vecs[j][r] > 0 {
                let coeff = Int::from_i64(vecs[j][r] as i64);
                terms.push(&x[j] * coeff);
            }
        }

        let lhs = if terms.is_empty() {
            Int::from_i64(0)
        } else {
            Int::add(&terms.iter().collect::<Vec<_>>())
        };

        let rhs = Int::from_i64(target_vec[r] as i64);
        opt.assert(&lhs.eq(&rhs));
    }

    let sum = Int::add(&x.iter().collect::<Vec<_>>());
    opt.minimize(&sum);

    match opt.check(&[]) {
        z3::SatResult::Sat => {
            if let Some(model) = opt.get_model() {
                let result = model.eval(&sum, true).unwrap();
                result.as_i64().unwrap()
            } else {
                panic!("no model");
            }
        }
        _ => panic!("no sol"),
    }
}

fn part_2(input: &Vec<String>) {
    let mut res = 0;
    for line in input {
        let (vecs, target_vec) = parse_line_into_vectors(line);
        let min_presses = compute_line(&vecs, &target_vec);
        res += min_presses;
    }
    println!("Part 2: {}", res);
}

fn main() {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }
    let input: Vec<String> = utils::read_lines(10);
    part_1(&input);
    part_2(&input);
}
