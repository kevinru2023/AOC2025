use aoc2025::utils;

fn main(){
    let mut curr_pos: i32 = 50;
    let mut password: u32 = 0;
    
    let v: Vec<String> = utils::read_lines(1);
    for line in &v{
        let mut num: i32 = line[1..line.len()].parse::<i32>().unwrap(); 
        if line.starts_with("L"){
            num *= -1;
        }
        curr_pos = (curr_pos + num) % 100;
        if curr_pos == 0 {
            password += 1;
        }
    }
    println!("Password: {}", password);
}