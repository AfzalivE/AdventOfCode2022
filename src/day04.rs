use std::ops::{Range, RangeFrom, RangeInclusive};

pub fn part1(input: String) {
    let inputs = input.split("\n");
    let option = inputs
        .map(|line| {
            let parts = line.split(",").collect::<Vec<_>>();
            let (first, second) = (parts[0], parts[1]);
            let range1 = to_range(first);
            let range2 = to_range(second);
            range2.contains(&range1.start()) && range2.contains(&range1.end()) ||
                range1.contains(&range2.start()) && range1.contains(&range2.end())
        })
        .filter(|x| { *x })
        .count();

    println!("sum: {}", option);
}

pub fn part2(input: String) {
    let inputs = input.split("\n");
    let option = inputs
        .map(|line| {
            let parts = line.split(",").collect::<Vec<_>>();
            let (first, second) = (parts[0], parts[1]);
            let range1 = to_range(first);
            let range2 = to_range(second);
            range2.contains(&range1.start()) || range2.contains(&range1.end()) ||
                range1.contains(&range2.start()) || range1.contains(&range2.end())
        })
        .filter(|x| { *x })
        .count();

    println!("sum: {}", option);
}

fn to_range(str: &str) -> RangeInclusive<i32> {
    let parts = str.split("-").collect::<Vec<_>>();
    let start = parts[0].parse().unwrap();
    let end = parts[1].parse().unwrap();
    RangeInclusive::new(start, end)
}
