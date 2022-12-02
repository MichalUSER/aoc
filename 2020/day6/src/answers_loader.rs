use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_answers() -> i16 {
    let mut count: i16 = 0;
    let mut group_count: i16 = 0;
    let mut answers: HashMap<char, i16> = HashMap::new();
    if let Ok(lines) = read_lines("answers.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let ips = String::from(ip);
                if ips.len() > 0 {
                    group_count += 1;
                    for c in ips.chars() {
                        let a = answers.get_mut(&c);
                        if let Some(a) = a {
                            *a += 1;
                        } else {
                            answers.insert(c, 1);
                        }
                    }
                } else {
                    //println!("Answers: {:?}", answers);
                    for a in answers.iter() {
                        if *a.1 >= group_count {
                            count += 1;
                        }
                    }
                    group_count = 0;
                    answers.clear();
                }
            }
        }
    }
    return count;
}

#[allow(dead_code)]
pub fn get_answers1() -> i16 {
    let mut count: i16 = 0;
    let mut answers: Vec<char> = Vec::new();
    if let Ok(lines) = read_lines("answers.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let ips = String::from(ip);
                if ips.len() > 0 {
                    for c in ips.chars() {
                        if !answers.contains(&c) {
                            count += 1;
                            answers.push(c);
                        }
                    }
                } else {
                    answers.clear();
                }
            }
        }
    }
    return count;
}
