fn main() {
    let mut sum: u64 = 0;
    let mut sums: Vec<u64> = vec![];
    include_str!("../input.txt").lines().for_each(|l| {
        if l == "" {
            sums.push(sum);
            sum = 0;
            return;
        }
        sum += l.parse::<u64>().unwrap();
    });
    sums.sort_unstable();
    let mut sum2 = 0;
    let len = sums.len();
    for i in len - 3..len {
        sum2 += sums[i];
    }
    println!("sum: {}", sum2);
}
