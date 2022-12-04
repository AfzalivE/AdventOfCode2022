use std::collections::HashSet;

pub fn part1(input: String) {
    let inputs = input.split("\n");
    let option = inputs
        .map(|x| {
            let (split1, split2) = x.split_at(x.len() / 2);
            let common = split1.chars().filter(|char| {
                split2.contains(&char.to_string())
            }).collect::<Vec<_>>()[0];
            let priority = get_priority(common);

            // println!("Line {} {}, common: {}, {}", split1, split2, common.to_string(), priority);
            priority
        })
        .sum::<i32>();

    println!("sum: {}", option);
}

fn get_priority(common: char) -> i32 {
    let priority;
    if common.is_lowercase() {
        priority = common as i32 - 'a' as i32 + 1;
    } else {
        priority = common as i32 - 'A' as i32 + 27;
    };
    priority
}

pub fn part2(input: String) {
    let inputs = input.split("\n").map(|x1| { String::from(x1) });
    let option: i32 = inputs
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| { chunk.to_vec() })
        .map(|x| {
            x.into_iter()
                .reduce(|acc_common_letters, item| {
                    let common_letters = item.chars().filter(|char| {
                        acc_common_letters.contains(*char)
                    }).collect::<HashSet<_>>();
                    let common_letter = common_letters.iter().collect::<String>();
                    common_letter
                }).unwrap()
        })
        .map(|common_letter| {
            get_priority(common_letter.chars().collect::<Vec<_>>()[0])
        })
        .sum();

    println!("index, sum: {}", option);
}