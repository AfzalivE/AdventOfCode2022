pub fn part1(input: String) {
    let inputs = input.split("\n\n");
    let option = inputs
        .into_iter()
        .enumerate()
        .map(|(i1, x2)| {
            let sum = x2.split("\n").map(|x1| { x1.parse::<u32>().unwrap() }).sum::<u32>();
            (i1.to_string(), sum)
        })
        .max_by(|(i2, x3), (i3, x4)| {
            x3.cmp(x4)
        });
    let (i, opt) = option.unwrap();

    println!("sum: {}", opt);
}

pub fn part1_b(input: String) {
    let inputs = input.split("\n\n");
    // TODO:fix this
    let mut option = inputs
        .into_iter()
        .enumerate()
        .map(|(i1, x2)| {
            let sum = x2.split("\n").map(|x1| { x1.parse::<u32>().unwrap() }).sum::<u32>();
            println!("index, sum: {}, {}", i1, sum);
            (i1.to_string(), sum)
        })
        .collect::<Vec<_>>();

    option.sort_by(|(_, x3), (_, x4)| {
        x4.cmp(x3)
    });

    // let top3 = option.take(3).unwrap();

    println!("index, sum: {:?}", option);
}