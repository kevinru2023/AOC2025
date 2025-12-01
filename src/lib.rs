pub mod utils {
    use std::fs;

    pub fn read_input(day: u8) -> String {
        let input_file = format!("inputs/day{:02}.txt", day);
        fs::read_to_string(input_file).expect("Failed to read input file")
    }
    
    pub fn read_lines(day: u8) -> Vec<String> {
        read_input(day)
            .lines()
            .map(|s| s.to_string())
            .collect()
    }
}
