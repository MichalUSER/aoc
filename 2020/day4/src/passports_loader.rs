use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::passport;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn load() -> Vec<passport::Passport> {
    let mut byr: String = String::new();
    let mut iyr: String = String::new();
    let mut eyr: String = String::new();
    let mut hgt: Vec<char> = Vec::new();
    let mut hcl: Vec<char> = Vec::new();
    let mut ecl: String = String::new();
    let mut pid: String = String::new();
    let mut passports_vec: Vec<passport::Passport> = Vec::new();
    if let Ok(lines) = read_lines("passports.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if ip.len() > 0 {
                    let line = String::from(ip);
                    let pairs: Vec<&str> = line.split(" ").collect();
                    for pair in pairs.iter() {
                        let keyandvalue: Vec<&str> = pair.split(':').collect();
                        match *keyandvalue.get(0).unwrap() {
                            "byr" => byr = String::from(*keyandvalue.get(1).unwrap()),
                            "iyr" => iyr = String::from(*keyandvalue.get(1).unwrap()),
                            "eyr" => eyr = String::from(*keyandvalue.get(1).unwrap()),
                            "hgt" => hgt = keyandvalue.get(1).unwrap().chars().collect(),
                            "hcl" => hcl = keyandvalue.get(1).unwrap().chars().collect(),
                            "ecl" => ecl = String::from(*keyandvalue.get(1).unwrap()),
                            "pid" => pid = String::from(*keyandvalue.get(1).unwrap()),
                            _ => ()
                        }
                    }
                } else {
                    let p = passport::Passport {
                        byr: byr,
                        iyr: iyr,
                        eyr: eyr,
                        hgt: hgt,
                        hcl: hcl,
                        ecl: ecl,
                        pid: pid
                    };
                    passports_vec.push(p);
                    byr = String::new();
                    iyr = String::new();
                    eyr = String::new();
                    hgt = Vec::new();
                    hcl = Vec::new();
                    ecl = String::new();
                    pid = String::new();
                }
            }
        }
    }
    return passports_vec;
}

#[allow(dead_code)]
pub fn load1() -> Vec<passport::Passport1> {
    let mut byr = false;
    let mut iyr = false;
    let mut eyr = false;
    let mut hgt = false;
    let mut hcl = false;
    let mut ecl = false;
    let mut pid = false;
    let mut passports_vec: Vec<passport::Passport1> = Vec::new();
    if let Ok(lines) = read_lines("passports.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if ip.len() > 0 {
                    let line = String::from(ip);
                    let pairs: Vec<&str> = line.split(" ").collect();
                    for pair in pairs.iter() {
                        let keyandvalue: Vec<&str> = pair.split(':').collect();
                        match *keyandvalue.get(0).unwrap() {
                            "byr" => byr = true,
                            "iyr" => iyr = true,
                            "eyr" => eyr = true,
                            "hgt" => hgt = true,
                            "hcl" => hcl = true,
                            "ecl" => ecl = true,
                            "pid" => pid = true,
                            _ => ()
                        }
                    }
                } else {
                    let p = passport::Passport1 {
                        byr: byr,
                        iyr: iyr,
                        eyr: eyr,
                        hgt: hgt,
                        hcl: hcl,
                        ecl: ecl,
                        pid: pid
                    };
                    passports_vec.push(p);
                    byr = false;
                    iyr = false;
                    eyr = false;
                    hgt = false;
                    hcl = false;
                    ecl = false;
                    pid = false;
                }
            }
        }
    }
    return passports_vec;
}
