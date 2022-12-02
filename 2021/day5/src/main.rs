use std::cmp::{max, min};
use std::{ops::Range, str::FromStr};

#[derive(Debug)]
struct Vent {
    x: Range<usize>,
    y: Range<usize>,
}

struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

fn parse() -> Vec<Vent> {
    include_str!("../input.txt")
        .lines()
        .map(|line| {
            let (start, end) = line.split_once(" -> ").unwrap();
            let start = Point::from_str(start).unwrap();
            let end = Point::from_str(end).unwrap();
            Vent {
                x: min(start.x, end.x)..(max(start.x, end.x) + 1),
                y: min(start.y, end.y)..(max(start.y, end.y) + 1),
            }
        })
        .collect()
}

impl Vent {
    fn is_straight(&self) -> bool {
        self.x.start + 1 == self.x.end || self.y.start + 1 == self.y.end
    }
}

/*.iter()
.map(|(first, second)| Vent {
    x: first.x.min(second.x)..first.x.max(second.x),
    y: first.y.min(second.y)..first.y.max(second.y),
})
.collect();*/
fn main() {
    let vents = parse();
    let (max_x, max_y) = vents.iter().fold((0, 0), |mut point, l| {
        if point.0 < l.x.end {
            point.0 = l.x.end;
        }
        if point.1 < l.y.end {
            point.1 = l.y.end;
        }
        point
    });

    for line in &vents {
        println!("{:?}", line);
    }

    let (_, count) = vents.iter().filter(|v| v.is_straight()).fold(
        (vec![vec![0; max_x]; max_y], 0),
        |mnc, v| {
            let (mut map, mut count) = mnc;
            for x in v.x.start..v.x.end {
                for y in v.y.start..v.y.end {
                    map[y][x] += 1;
                    if map[y][x] == 2 {
                        count += 1;
                    }
                }
            }
            return (map, count);
        },
    );
    println!("count: {}", count);
}
