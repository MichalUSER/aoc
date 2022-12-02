pub struct Passport {
    pub byr: String,
    pub iyr: String,
    pub eyr: String,
    pub hgt: Vec<char>,
    pub hcl: Vec<char>,
    pub ecl: String,
    pub pid: String
}

fn byr_valid(byr: &String) -> bool {
    if byr.len() == 4 {
        let year: i32 = match byr.parse() {
            Ok(year) => year,
            Err(_) => return false
        };
        if year >= 1920 && year <= 2002 {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn iyr_valid(iyr: &String) -> bool {
    if iyr.len() == 4 {
        let year: i32 = match iyr.parse() {
            Ok(year) => year,
            Err(_) => return false
        };
        if year >= 2010 && year <= 2020 {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn eyr_valid(eyr: &String) -> bool {
    if eyr.len() == 4 {
        let year: i32 = match eyr.parse() {
            Ok(year) => year,
            Err(_) => return false
        };
        if year >= 2020 && year <= 2030 {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn hgt_valid(hgt: &Vec<char>) -> bool {
    let hgtlen = hgt.len();
    if hgtlen > 3 {
        let first: char = *hgt.get(hgtlen-2).unwrap();
        let second: char = *hgt.get(hgtlen-1).unwrap();
        if first == 'c' && second == 'm' {
            let mut num_str = String::new();
            for i in 0..hgtlen-2 {
                num_str.push(hgt[i]);
            }
            let num: u16 = match num_str.parse() {
                Ok(num) => num,
                Err(_) => return false
            };
            if num >= 150 && num <= 193 {
                true
            } else {
                false
            }
        } else if first == 'i' && second == 'n' {
            let mut num_str = String::new();
            for i in 0..hgtlen-2 {
                num_str.push(hgt[i]);
            }
            let num: u16 = match num_str.parse() {
                Ok(num) => num,
                Err(_) => return false
            };
            if num >= 59 && num <= 76 {
                true
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

fn hcl_valid(hcl: &Vec<char>) -> bool {
    let hcllen = hcl.len();
    if hcllen == 7 {
        if hcl[0] == '#' {
            for i in 1..hcllen {
                if !hcl[i].is_numeric() {
                    match hcl[i] {
                        'a' => (), 'b' => (), 'c' => (), 'd' => (), 'e' => (), 'f' => (),
                        _ => return false
                    }
                }
            }
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn ecl_valid(ecl: &String) -> bool {
    match ecl.as_str() {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false
    }
}

fn pid_valid(pid: &String) -> bool {
    if pid.len() == 9 {
        true
    } else {
        false
    }
}

impl Passport {
    pub fn valid(&self) -> bool {
        if byr_valid(&self.byr) && iyr_valid(&self.iyr) && eyr_valid(&self.eyr) && hgt_valid(&self.hgt) &&
        hcl_valid(&self.hcl) && ecl_valid(&self.ecl) && pid_valid(&self.pid) {
            true
        } else {
            false
        }
    }
}

pub struct Passport1 {
    pub byr: bool,
    pub iyr: bool,
    pub eyr: bool,
    pub hgt: bool,
    pub hcl: bool,
    pub ecl: bool,
    pub pid: bool
}

impl Passport1 {
    #[allow(dead_code)]
    pub fn valid(&self) -> bool {
        if self.byr && self.iyr && self.eyr && self.hgt && self.hcl && self.ecl && self.pid {
            true
        } else {
            false
        }
    }
}
