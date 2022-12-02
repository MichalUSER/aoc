// dumbest solution ever (only works for part 1 cuz not enough ram)
/*
for _ in 0..256 {
        let mut new_list: Vec<i8> =
            (0..fish_intervals.len() - 1).fold(Vec::new(), |mut new_list, i| {
                if fish_intervals[i] == 0 {
                    fish_intervals[i] = 6;
                    new_list.push(8)
                } else {
                    fish_intervals[i] -= 1;
                }
                new_list
            });
        fish_intervals.append(&mut new_list);
    }?
*/

fn main() {
    // "3,4,3,1,2"
    // include_str!("../input.txt")
    let mut f: [i128; 9] = include_str!("../input.txt")
        .trim()
        .split(',')
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .fold([0; 9], |mut a, n| {
            a[n] += 1;
            a
        });
    // println!("days_with_intervals: {:?}", fish_days);
    for _ in 0..256 {
        f = [f[1], f[2], f[3], f[4], f[5], f[6], f[7] + f[0], f[8], f[0]];
    }
    // println!("fish intervals: {:?}", fish_intervals);
    println!("count: {}", f.iter().sum::<i128>());
}
