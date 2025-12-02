use aoc2025::utils;

fn part_1(start_num: u64, end_num:u64, invalid_ids: &mut u64){
    for num in start_num..=end_num {
        let str_num = num.to_string();
        let str_num_len = str_num.chars().count();
        if str_num_len % 2 == 0 {
            let mid = str_num_len / 2;
            let (left, right) = str_num.split_at(mid);
            if left == right {
                *invalid_ids += num;
            }
        }
    }
}

fn part_2(start_num: u64, end_num: u64, invalid_ids: &mut u64){
    for num in start_num..=end_num {
        let str_num = num.to_string();
        // check if string repeats itself: https://www.geeksforgeeks.org/python/python-check-if-string-repeats-itself/
        let doubled = format!("{}{}", str_num, str_num);
        let slice = &doubled[1..doubled.len()-1];
        if slice.contains(&str_num) {
            *invalid_ids += num;
        }
    }
}

fn solution() {
    let s = utils::read_input(2);
    let ranges = s.trim().split(",");
    let mut invalid_ids = 0;
    let mut invalid_ids_2 = 0;
    for range in ranges {
        // format is "start-end"
        let (start, end) = range.split_once("-").unwrap();
        let start_num = start.parse::<u64>().unwrap();
        let end_num = end.parse::<u64>().unwrap();

        part_1(start_num, end_num, &mut invalid_ids);
        part_2(start_num, end_num, &mut invalid_ids_2);
    }
    println!("Invalid IDs: {}", invalid_ids);
    println!("Invalid IDs 2: {}", invalid_ids_2);
}

fn main() {
    solution();
}

