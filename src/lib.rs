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
    
    // for day 8, SO AI cause this was a headache (mainly union func)
    pub struct DSU {
        parent: Vec<usize>,
        size: Vec<usize>,
    }
    
    impl DSU {
        pub fn new(n: usize) -> Self {
            Self {
                parent: (0..n).collect(),
                size: vec![1; n],
            }
        }
    
        pub fn find(&mut self, x: usize) -> usize {
            if self.parent[x] != x {
                let root = self.find(self.parent[x]);
                self.parent[x] = root;
            }
            self.parent[x]
        }
    
        pub fn union(&mut self, a: usize, b: usize) {
            let mut root_a = self.find(a);
            let mut root_b = self.find(b);
            if root_a != root_b {
                if self.size[root_a] < self.size[root_b] {
                    std::mem::swap(&mut root_a, &mut root_b); // wow.
                }
                self.parent[root_b] = root_a;
                self.size[root_a] += self.size[root_b];
            }
        }
        
        pub fn connected(&mut self, a: usize, b: usize) -> bool {
            self.find(a) == self.find(b)
        }
        
        pub fn component_size(&mut self, x: usize) -> usize {
            let root = self.find(x);
            self.size[root]
        }
    }
}
