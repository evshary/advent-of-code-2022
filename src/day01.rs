use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;

fn main() {
	let mut largest_elf = 0;
	let mut tmp_elf = 0;
    if let Ok(lines) = read_lines("input/day01.txt") {
        for line in lines {
            if let Ok(content) = line {
                if content == "" {
					largest_elf = cmp::max(largest_elf, tmp_elf);
					tmp_elf = 0;
				} else {
					tmp_elf += content.parse::<i32>().unwrap();
				}
            }
        }
		println!("largest_elf is {}", largest_elf);
    } else {
		println!("Unable to read file.");
	}
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
