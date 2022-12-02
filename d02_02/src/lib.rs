#[derive(Clone, Copy)]
enum PlayerMove {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

#[derive(Clone, Copy)]
enum Outcome {
    LOST = 0,
    DRAW = 3,
    WON = 6,
}

/*
single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won)
*/

fn score(lhs: PlayerMove, out: Outcome) -> i32 {
    out as i32 + match out {
        Outcome::LOST => match lhs {
            PlayerMove::ROCK => PlayerMove::SCISSORS,
            PlayerMove::PAPER => PlayerMove::ROCK,
            PlayerMove::SCISSORS => PlayerMove::PAPER,
        },
        Outcome::DRAW => lhs,
        Outcome::WON => match lhs {
            PlayerMove::ROCK => PlayerMove::PAPER,
            PlayerMove::PAPER => PlayerMove::SCISSORS,
            PlayerMove::SCISSORS => PlayerMove::ROCK,
        },
    } as i32
}

fn decode_move(val: &str) -> PlayerMove {
    match val.chars().next().unwrap() {
        'A' => PlayerMove::ROCK,
        'B' => PlayerMove::PAPER,
        'C' => PlayerMove::SCISSORS,
        _ => unreachable!(),
    }
}

fn decode_draw(val: &str) -> Outcome {
    match val.chars().next().unwrap() {
        'X' => Outcome::LOST,
        'Y' => Outcome::DRAW,
        'Z' => Outcome::WON,
        _ => unreachable!(),
    }
}

pub fn solve() -> i32 {
    include_str!("../input.txt")
        .lines()
        .map(|l| {
            let mut v = l.split_ascii_whitespace();
            score(decode_move(v.next().unwrap()), decode_draw(v.next().unwrap()))
        }).sum::<i32>()
}