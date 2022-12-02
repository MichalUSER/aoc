use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn bag_content(s: String) -> Vec<String> {
    let mut bags: Vec<String> = Vec::new();
    let split_multiple: Vec<&str> = s.split(", ").collect();
    for e in split_multiple.into_iter() {
        let mut es = String::from(e);
        let e_vec: Vec<char> = es.chars().collect();
        if e_vec[0] == '1' {
            if e_vec[es.len()-1] == '.' {
                es.drain(0..2);
                let l = es.len();
                es.drain(l-5..l);
            } else {
                es.drain(0..2);
                let l = es.len();
                es.drain(l-4..l);
            }
            bags.push(es);
        } else if e_vec[0] != 'n' {
            if e_vec[es.len()-1] == '.' {
                es.drain(0..2);
                let l = es.len();
                es.drain(l-6..l);
            } else {
                es.drain(0..2);
                let l = es.len();
                es.drain(l-5..l);
            }
            bags.push(es);
        }
    }
    return bags
}

pub fn get_bags() -> HashMap<String, Vec<String>> {
    let mut bags: HashMap<String, Vec<String>> = HashMap::new();
    if let Ok(lines) = read_lines("bags.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let bag: Vec<&str> = ip.split(" contain ").collect();
                let bc = bag_content(String::from(bag[1]));
                if bc.len() > 0 {
                    let mut bag_name = String::from(bag[0]);
                    let l = bag_name.len();
                    bag_name.drain(l-5..l);
                    bags.insert(bag_name, bc);
                }
            }
        }
    }
    return bags;
}
