/*
fn get_points(choice1: &str, choice2: &str) -> i32 {
    let outcome = match (choice1, choice2) {
        // Draw
        ("A", "X") => 3,
        ("B", "Y") => 3,
        ("C", "Z") => 3,
        // Win
        ("A", "Y") => 6,
        ("B", "Z") => 6,
        ("C", "X") => 6,
        // Loss
        ("A", "Z") => 0,
        ("B", "X") => 0,
        ("C", "Y") => 0,
        _ => 0,
    };
    let shape = match choice2 {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    };

    outcome + shape
}

fn main() {
    let sum = include_str!("../input.txt").lines().fold(0, |acc, l| {
        let strategy: Vec<&str> = l.split(' ').collect();
        let choice1 = strategy[0];
        let choice2 = strategy[1];
        acc + get_points(choice1, choice2)
    });
    println!("sum: {}", sum);
}
*/
