use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_priority(a: &str, b: &str) -> u32 {
    let mut idx = 0;
    let mut v = [false; 52];
    for c in a.chars() {
        idx = c as usize;
        idx -= if idx >= 97 { 97 } else { 65 - 26 };
        v[idx] = true 
    }
    for c in b.chars() {
        idx = c as usize;
        idx -= if idx >= 97 { 97 } else { 65 - 26 };
        if v[idx] == true {
            break;
        }
    }
    //println!("{}", idx + 1);
    (idx + 1) as u32
}

fn main() {
    let mut priority = 0;
    if let Ok(lines) = read_lines("input/day03.txt") {
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
