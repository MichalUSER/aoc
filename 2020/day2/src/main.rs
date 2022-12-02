use std::time::{Instant};

mod passwords_loader;
mod password;

fn main() {
    let now = Instant::now();
    let passwords = passwords_loader::get_passwords();
    let mut counter = 0;
    for p in passwords {
        if let Some(valid) = p.validate_password2() {
            if valid {
                counter += 1;
            }
        }
    }
    println!("Correct passwords: {}", counter);
    println!("elapsed {}", now.elapsed().as_millis())

    //let p = password::Password {
        //text: String::from("ccccccccc"),
        //least_count: 2,
        //most_count: 9,
        //letter: 'c'
    //};
    //println!("Valid: {}", p.validate_password2().unwrap());
}
