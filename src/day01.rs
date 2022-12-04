pub fn part1(input: String) {
    let inputs = input.split("\n\n");
    let option = inputs
        .map(|x2| {
            let sum = x2.split("\n").map(|x1| { x1.parse::<u32>().unwrap() }).sum::<u32>();
            sum
        })
        .max_by(| x3, x4| {
            x3.cmp(x4)
        });

    println!("sum: {}", option.unwrap());
}

pub fn part2(input: String) {
    let inputs = input.split("\n\n");
    let mut option = inputs
        .map(|x| {
            let sum = x.split("\n").map(|x1| { x1.parse::<u32>().unwrap() }).sum::<u32>();
            sum
        })
        .collect::<Vec<_>>();

    option.sort_by(|x3, x4| { x4.cmp(x3) });

    let sum_top3 = option.into_iter().take(3).sum::<u32>();

    println!("index, sum: {:?}", sum_top3);
}