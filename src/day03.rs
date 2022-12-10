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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1() {
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

fn get_priority2(x: &str, y: &str, z: &str) -> u32 {
    let mut idx = 0;
    let mut v = [0; 52];
    for c in x.chars() {
        if c == '\r' || c == '\n' { break; }
        idx = c as usize;
        idx -= if idx >= 97 { 97 } else { 65 - 26 };
        v[idx] = 1; 
    }
    for c in y.chars() {
        if c == '\r' || c == '\n' { break; }
        idx = c as usize;
        idx -= if idx >= 97 { 97 } else { 65 - 26 };
        if v[idx] == 1 {
            v[idx] += 2;
        }
    }
    for c in z.chars() {
        idx = c as usize;
        idx -= if idx >= 97 { 97 } else { 65 - 26 };
        if v[idx] == 3 {
            break;
        }
    }
    //println!("{}", idx + 1);
    (idx + 1) as u32
}

fn read_file(filename: &str) -> io::Result<io::BufReader<File>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

fn part2() {
    let mut priority = 0;
    let mut reader = read_file("input/day03.txt").unwrap();
    let mut line1 = String::new();
    let mut line2 = String::new();
    let mut line3 = String::new();
    loop {
        if reader.read_line(&mut line1).unwrap() == 0 { break; }
        if reader.read_line(&mut line2).unwrap() == 0 { break; }
        if reader.read_line(&mut line3).unwrap() == 0 { break; }
        priority += get_priority2(&line1, &line2, &line3);
        line1.clear();
        line2.clear();
        line3.clear();
    }
    println!("priority = {}", priority);
}

fn main() {
    part1();
    part2();
}

