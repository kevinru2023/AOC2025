use aoc2025::utils;
use array2d::Array2D;
use std::env;

fn parse_input(input: &[String]) -> (Array2D<u64>, Vec<String>) {
    let mut rows: Vec<Vec<u64>> = Vec::new();
    let n = input.len();
    for i in 0..n - 1 {
        // don't take last element
        let line = &input[i];
        let nums: Vec<u64> = line
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        rows.push(nums);
    }

    let numbers = Array2D::from_rows(&rows).unwrap();

    let operations: Vec<String> = input[n - 1]
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    (numbers, operations)
}


// WARNING: Vibe coded hard here, I was so done with trying to get rust 
// to parse and keep the white space correctly. I'll probably refactor this later, but 
// it's currently 3:05 am, got nerdsniped by this problem hard. 
// Parser that extracts each problem as a vertical column (again this was vibe coded), 
// Returns: Vec of problems, where each problem is Vec<String> containing [num1, num2, num3, operation]
fn parser(input: &[String]) -> Vec<Vec<String>> {
    if input.is_empty() {
        return Vec::new();
    }

    // Find the maximum line length to determine how many columns we need to check
    let max_len = input.iter().map(|line| line.len()).max().unwrap_or(0);

    let mut problems: Vec<Vec<String>> = Vec::new();
    let mut col_idx = 0;

    while col_idx < max_len {
        // Check if this column position is all spaces across all rows
        let is_separator = input
            .iter()
            .all(|line| line.chars().nth(col_idx).map_or(true, |c| c == ' '));

        if is_separator {
            col_idx += 1;
            continue;
        }

        // Extract the problem starting at this column
        let mut problem_strings: Vec<String> = Vec::new();
        let mut problem_width = 0;

        // For each row, extract the characters for this problem
        for row in input {
            let mut chars_in_problem = String::new();
            let mut current_col = col_idx;

            // Collect characters until we hit a separator column or end of line
            while current_col < row.len() {
                // Check if this column position is a separator across all rows
                let is_sep_col = input
                    .iter()
                    .all(|line| line.chars().nth(current_col).map_or(true, |c| c == ' '));

                if is_sep_col {
                    break;
                }

                let ch = row.chars().nth(current_col).unwrap_or(' ');
                chars_in_problem.push(ch);
                current_col += 1;
            }

            // Track the maximum width of this problem
            problem_width = problem_width.max(chars_in_problem.len());

            // Keep the string as-is, preserving all whitespace
            problem_strings.push(chars_in_problem);
        }

        problems.push(problem_strings);

        // Move to the next problem (skip past this problem's width)
        col_idx += problem_width;
    }

    problems
}

fn part_1(numbers: Array2D<u64>, operations: Vec<String>) -> u64 {
    let mut res = 0;
    let mut op_idx = 0;
    for column_iter in numbers.columns_iter() {
        let mut curr_res: u64 = 0;
        for ele in column_iter {
            if &operations[op_idx] == "+" {
                curr_res += *ele;
            } else if &operations[op_idx] == "*" && curr_res != 0 {
                curr_res *= *ele;
            } else {
                curr_res = *ele;
            }
        }
        op_idx += 1;
        res += curr_res;
    }
    res
}

fn part_2(parsed_cols: Vec<Vec<String>>) -> u64 {
    let mut res = 0;
    for col in parsed_cols {
        let mut curr = 0;
        let n = col.len();
        let m = col[0].len();
        let op = *&col[n - 1].trim();
        for i in 0..m {
            let mut curr_num = String::new();
            for j in 0..n-1{ // skip last value
                if let Some(ch) = col[j].chars().nth(i) {
                    if ch != ' ' {
                        curr_num.push(ch);
                    }
                }
            }
            if !curr_num.is_empty() {
                let num = curr_num.parse::<u64>().unwrap();
                if op == "+" {
                    curr += num;
                } else if op == "*" && curr != 0 {
                    curr *= num;
                } else {
                    curr = num;
                }
            }
        }
        res += curr;
        
    }
    res
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let input: Vec<String> = utils::read_lines(6);
    // now with that input lets parse it in two ways
    // one that gives us the direct numbers and operations
    // and another that gives us the columns of the input i.e. for input:
    // 123 328  51 64
    //  45 64  387 23
    //   6 98  215 314
    // *   +   *   +
    // output would give a vec like [["123", " 45", "  6", "*   "], ["328", "64 ", "98 ", "+  "], ...]
    // we can leverage the fact that each col is seperated by a 4 chars

    let (numbers, operations) = parse_input(&input);
    let parsed_columns: Vec<Vec<String>> = parser(&input); // this made me wanna kms
    let part_1_res = part_1(numbers, operations);
    let part_2_res = part_2(parsed_columns);
    println!("Part 1: {}", part_1_res);
    println!("Part 2: {}", part_2_res);
}
