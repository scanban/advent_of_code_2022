#[derive(Clone, Copy)]
enum PlayerMove {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

enum Outcome {
    LOST = 0,
    DRAW = 3,
    WON = 6,
}


/*
single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won)
*/

fn score(lhs: PlayerMove, rhs: PlayerMove) -> i32 {
    rhs as i32 + match lhs {
        PlayerMove::ROCK => match rhs {
            PlayerMove::ROCK => Outcome::DRAW,
            PlayerMove::PAPER => Outcome::WON,
            PlayerMove::SCISSORS => Outcome::LOST,
        },
        PlayerMove::PAPER => match rhs {
            PlayerMove::ROCK => Outcome::LOST,
            PlayerMove::PAPER => Outcome::DRAW,
            PlayerMove::SCISSORS => Outcome::WON,
        }
        PlayerMove::SCISSORS => match rhs {
            PlayerMove::ROCK => Outcome::WON,
            PlayerMove::PAPER => Outcome::LOST,
            PlayerMove::SCISSORS => Outcome::DRAW,
        }
    } as i32
}

fn decode(val: &str) -> PlayerMove {
    match val.chars().next().unwrap() {
        'A' | 'X' => PlayerMove::ROCK,
        'B' | 'Y' => PlayerMove::PAPER,
        'C' | 'Z' => PlayerMove::SCISSORS,
        _ => unreachable!(),
    }
}

pub fn solve() -> i32 {
    include_str!("../input.txt")
        .lines()
        .map(|l| {
            let mut v = l.split_ascii_whitespace();
            score(decode(v.next().unwrap()), decode(v.next().unwrap()))
        }).sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_solve() {
        println!("{}", solve());
    }
}