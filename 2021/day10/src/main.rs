use std::collections::HashMap;

fn get_opposite(c: char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        '[' => ']',
        ']' => '[',
        _ => '0',
    }
}

fn main() {
    let lines: Vec<Vec<char>> = include_str!("../example2.txt")
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut score = 0;

    for l in lines {
        let count = l.iter().fold(HashMap::<char, i32>::new(), |mut count, c| {
            *count.entry(*c).or_insert(0) += 1;
            count
        });
        println!("count: {:?}", count);
        let mut skip: Vec<char> = Vec::new();
        for (key, val) in count.iter() {
            if skip.contains(key) {
                continue;
            }
            let opposite_count = match count.get(&get_opposite(*key)) {
                Some(v) => *v,
                None => *val,
            };
            skip.push(get_opposite(*key));
            let err_count = (val - opposite_count).abs();
            if err_count > 0 {
                match key {
                    ')' => score += 3 * err_count,
                    ']' => score += 57 * err_count,
                    '}' => score += 1197 * err_count,
                    '>' => score += 25137 * err_count,
                    _ => (),
                }
            }
        }
    }

    println!("score: {}", score);
}
