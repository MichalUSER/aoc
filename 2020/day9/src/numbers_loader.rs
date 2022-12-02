use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_numbers() -> Vec<i64> {
    let mut numbers: Vec<i64> = Vec::new();
    if let Ok(lines) = read_lines("numbers.txt") {
        for line in lines {
            if let Ok(ip) = line {
                numbers.push(ip.parse().unwrap());
            }
        }
    }
    return numbers;
}
