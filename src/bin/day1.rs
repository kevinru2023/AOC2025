use aoc2025::utils;
fn main(){
    let mut curr_pos: i32 = 50;
    let mut password: i32 = 0;
    
    let v: Vec<String> = utils::read_lines(1);
    for line in &v{
        let num: i32 = line[1..line.len()].parse::<i32>().unwrap(); 
        for _ in 0..num {
            if line.starts_with("L"){
                curr_pos -= 1;
            } else{
                curr_pos += 1;
            }
            
            // check bounds
            if curr_pos == -1 {
                curr_pos = 99;
            } else if curr_pos == 100 {
                curr_pos = 0;
            }
            if curr_pos == 0 {
                password += 1;
            }
        }
    }
    println!("Password: {}", password);
}