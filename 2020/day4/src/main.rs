mod passports_loader;
mod passport;

use std::time::Instant;

//println!("First: {}", ps.get(0).unwrap());
//let p = passport::Passport {
//byr: true,
//iyr: true,
//eyr: true,
//hgt: true,
//hcl: true,
//ecl: true,
//pid: false
//};
//println!("Valid: {}", p.valid());
fn main() {
    let now = Instant::now();
    let ps: Vec<passport::Passport> = passports_loader::load();
    //let p = passport::Passport {
        //byr: String::from("1980"),
        //iyr: String::from("2012"),
        //eyr: String::from("2030"),
        //hgt: vec!['7', '4', 'i', 'n'],
        //hcl: vec!['#', '6', '2', '3', 'a', '0', 'f'],
        //ecl: String::from("grn"),
        //pid: String::from("087499704")
        //pid: vec!['0', '8', '7', '4', '9', '9', '7', '0', '4']
    //};
    //println!("Valid: {}", p.valid());
    let mut counter = 0;
    for p in ps {
        if p.valid() {
            counter += 1;
        }
    }
    println!("Valid passports: {}", counter);
    println!("Elapsed: {}ms", now.elapsed().as_millis());
}
