use aoc2025::utils;

fn part_1 (s: Vec<String>){
    let mut res: u32 = 0;
    for line in s {
        let bytes = line.as_bytes();
        let mut max_val: u32 = 0;
        for i in 0..bytes.len(){
            let ch = bytes[i] as char;
            for j in i+1..bytes.len(){ 
                let tmp = format!("{}{}", ch, bytes[j] as char);
                if tmp.parse::<u32>().is_ok() && tmp.parse::<u32>().unwrap() > max_val {
                    max_val = tmp.parse::<u32>().unwrap();
                }
            }
        }
        res += max_val;
    }
    println!("{}", res);
}

fn part_2(s: Vec<String>){
    let mut res: u64 = 0;
    for line in s {
        let mut stack: Vec<u8> = Vec::new(); // montonic increasing stack
        let mut removals = line.len() - 12;
        
        for i in 0..line.len(){ 
            let ch = line.chars().nth(i).unwrap() as u8; // wtf
            while !stack.is_empty() && *stack.last().unwrap() < ch && removals > 0 {
                stack.pop();
                removals -= 1;
            }
            stack.push(ch);
        }
        
        let mut num: String = String::new();
        for i in 0..12 {
            num.push(stack[i] as char);
        }
        res += num.parse::<u64>().unwrap();
    }
    println!("{}", res);
}

fn main(){
    part_1(utils::read_lines(3));
    part_2(utils::read_lines(3));
}