use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn get_grade(a: &str, b: &str) -> u32 {
   let mappings = HashMap::from([
      ("X", 0),
      ("Y", 1),
      ("Z", 2),
      ("A", 0),
      ("B", 1),
      ("C", 2),
    ]); 
   //   X Y Z
   // A 3 6 0
   // B 0 3 6
   // C 6 0 3
   let win_table = vec![
        [3, 6, 0],
        [0, 3, 6],
        [6, 0, 3],
   ];
   (win_table[mappings[a]][mappings[b]] + 1 + mappings[b]).try_into().unwrap()
}

fn main() {
    let mut total_grade = 0;
    if let Ok(lines) = read_lines("input/day02.txt") {
        for line in lines {
            if let Ok(content) = line {
                let mut iter = content.split_whitespace();
                let elf_shape = iter.next().unwrap();
                let my_shape = iter.next().unwrap();
                total_grade += get_grade(elf_shape, my_shape);
            }
        }
        println!("Total grade = {}", total_grade);
    } else {
		println!("Unable to read file.");
	}
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
