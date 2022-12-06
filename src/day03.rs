use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn get_priority(a: &str, b: &str) -> u32 {
    1
}

fn main() {
    let mut priority = 0;
    if let Ok(lines) = read_lines("input/day03_test.txt") {
        for line in lines {
            if let Ok(content) = line {
                let len = content.len();
                priority += get_priority(&content[0..(len/2)], &content[len/2..len]);
            }
        }
        println!("priority = {}", priority);
    } else {
		println!("Unable to read file.");
	}
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
