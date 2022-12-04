fn get_points(choice: &str, desired: &str) -> i32 {
    let choice2 = match desired {
        // Lose
        "X" => match choice {
            "A" => "Z",
            "B" => "X",
            "C" => "Y",
            _ => ""
        },
        // Draw
        "Y" => match choice {
            "A" => "X",
            "B" => "Y",
            "C" => "Z",
            _ => ""
        },
        // Win
        "Z" => match choice {
            "A" => "Y",
            "B" => "Z",
            "C" => "X",
            _ => ""
        },
        _ => "",
    };
    let outcome = match desired {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => 0,
    };
    let shape = match choice2 {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0
    };

    outcome + shape
}

fn main() {
    let sum = include_str!("../example.txt").lines().fold(0, |acc, l| {
        let strategy: Vec<&str> = l.split(' ').collect();
        let choice = strategy[0];
        let desired = strategy[1];
        acc + get_points(choice, desired)
    });
    println!("sum: {}", sum);
}
