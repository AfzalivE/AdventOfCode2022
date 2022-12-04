use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use crate::day02::Shape::Rock;
use crate::day02::Shape::Scissors;
use crate::day02::Shape::Paper;

pub fn part1(input: String) {
    let inputs = input.split("\n");
    let option = inputs
        .map(|x| {
            let shapes = x.split(" ").into_iter().collect::<Vec<_>>();
            let (opponent_shape, selected_shape) = (to_shape(shapes[0]), to_shape(shapes[1]));
            // println!("opp, mine: {}, {}", opponent_shape, selected_shape);
            let shape_score = selected_shape.score();
            let win_score = get_win_score(opponent_shape, selected_shape);
            // println!("shape, win: {}, {}", shape_score, win_score);
            shape_score + win_score
        })
        .sum::<u32>();

    println!("sum of scores: {}", option);
}

pub fn part2(input: String) {
    let inputs = input.split("\n");
    let option = inputs
        .map(|x| {
            let shapes = x.split(" ").into_iter().collect::<Vec<_>>();
            let (opponent_shape, result) = (to_shape(shapes[0]), shapes[1]);
            // println!("opp, result: {}, {}", opponent_shape, result);
            let my_shape = get_my_shape(opponent_shape, result);
            let win_score = get_win_score(opponent_shape, my_shape);
            // println!("shape, win: {}, {}", my_shape.score(), win_score);
            my_shape.score() + win_score
        })
        .sum::<u32>();

    println!("sum of scores: {}", option);
}

fn get_my_shape(opponent_shape: Shape, result: &str) -> Shape {
    match (opponent_shape, result) {
        (Rock, "X") | (Paper, "Z") | (Scissors, "Y") => Scissors,
        (Rock, "Y") | (Paper, "X") | (Scissors, "Z") => Rock,
        (Rock, "Z") | (Paper, "Y") | (Scissors, "X") => Paper,
        _ => { panic!("Unknown combination, opp: {}, result: {}", opponent_shape, result) }
    }
}

fn get_win_score(opponent_shape: Shape, selected_shape: Shape) -> u32 {
    match (selected_shape, opponent_shape) {
        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
        (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => 6,
        _ => 0,
    }
}

trait Score {
    fn score(&self) -> u32;
}

enum Shape { Rock, Paper, Scissors }

impl Display for Shape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Rock => write!(f, "Rock"),
            Scissors => write!(f, "Scissors"),
            Paper => write!(f, "Paper")
        }
    }
}

impl Score for Shape {
    fn score(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

impl Clone for Shape {
    fn clone(&self) -> Self {
        match self {
            Rock => Rock,
            Scissors => Scissors,
            Paper => Paper
        }
    }
}

impl Copy for Shape {}

/** Handle X, Y, Z for part 1 **/
fn to_shape(shape: &str) -> Shape {
    return match shape {
        "A" | "X" => Rock,
        "B" | "Y" => Paper,
        "C" | "Z" => Scissors,
        _ => { panic!("Unknown shape {}", shape); }
    };
}
