use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut elf_vec = Vec::new();
    let mut tmp_elf = 0;
    if let Ok(lines) = read_lines("input/day01.txt") {
        for line in lines {
            if let Ok(content) = line {
                if content == "" {
                    elf_vec.push(tmp_elf);
					tmp_elf = 0;
				} else {
					tmp_elf += content.parse::<i32>().unwrap();
				}
            }
        }
        elf_vec.sort();
        elf_vec.reverse();
		println!("The largest elf is {}", elf_vec[0]);
        println!("The first 3 largest elf is {}", elf_vec[0]+elf_vec[1]+elf_vec[2]);
    } else {
		println!("Unable to read file.");
	}
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
