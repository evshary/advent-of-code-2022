use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn chk_full_overlap(a: u32, b: u32, c: u32, d: u32) -> bool {
    if (a <= c && d <= b) || (c <= a && b <= d) {
        true
    } else {
        false
    }
}

fn chk_overlap(a: u32, b: u32, c: u32, d: u32) -> bool {
    if (c <= a && a <= d) || (a <= c && c <= b) {
        true
    } else {
        false
    }
}

fn main() {
    let mut full_overlap_num = 0;
    let mut overlap_num = 0;
    if let Ok(lines) = read_lines("input/day04.txt") {
        for line in lines {
            if let Ok(content) = line {
                let values: Vec<_> = content.split(",").collect();
                let ranges1: Vec<_> = values[0].split("-").collect();
                let ranges2: Vec<_> = values[1].split("-").collect();
                if chk_full_overlap(ranges1[0].parse().unwrap(),
                                    ranges1[1].parse().unwrap(),
                                    ranges2[0].parse().unwrap(),
                                    ranges2[1].parse().unwrap()) {
                    full_overlap_num += 1;
                }
                if chk_overlap(ranges1[0].parse().unwrap(),
                               ranges1[1].parse().unwrap(),
                               ranges2[0].parse().unwrap(),
                               ranges2[1].parse().unwrap()) {
                    overlap_num += 1;
                }
            }
        }
        println!("Total full overlap = {}", full_overlap_num);
        println!("Total overlap = {}", overlap_num);
    } else {
		println!("Unable to read file.");
	}
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
