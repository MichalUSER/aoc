fn main() {
    let sum = include_str!("../input.txt").lines().fold(0, |acc, l| {
        let sum2 = l.chars().fold(String::new(), |mut a, c| {
            if c >= '\u{0030}' && c <= '\u{0039}' {
                a.push(c)
            }
            a
        });
        let nums = sum2.chars().nth(0).unwrap().to_string()
            + sum2.chars().last().unwrap().to_string().as_str();
        acc + nums.parse::<i32>().unwrap()
    });
    println!("sum is {}", sum);
}
