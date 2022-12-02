use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::password;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_passwords() -> Vec<password::Password> {
    let mut passwords_vec: Vec<password::Password> = Vec::new();
    if let Ok(lines) = read_lines("passwords.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let items: Vec<&str> = ip.split(" ").collect();
                let count: Vec<&str> = items.get(0).unwrap().split('-').collect();
                let current_letter: Vec<char> = String::from(*items.get(1).unwrap()).chars().collect();
                let current_text = items.get(2).unwrap();
                let p = password::Password {
                    text: String::from(*current_text),
                    least_count: String::from(*count.get(0).unwrap()).parse().unwrap(),
                    most_count: String::from(*count.get(1).unwrap()).parse().unwrap(),
                    letter: *current_letter.get(0).unwrap()
                };
                passwords_vec.push(p);
            }
        }
    }
    return passwords_vec;
}
