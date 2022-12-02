use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_path() -> Vec<Vec<u8>> {
    let mut path_vec: Vec<Vec<u8>> = Vec::new();
    if let Ok(lines) = read_lines("path.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let mut line: Vec<u8> = Vec::new();
                let chars: Vec<char> = String::from(ip).chars().collect();
                for _ in 0..108 {
                    for c in &chars {
                        if *c == '.' {
                            line.push(0);
                        } else {
                            line.push(1);
                        }
                    }
                }
                path_vec.push(line);
            }
        }
    }
    return path_vec;
}
