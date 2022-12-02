use itertools::sorted;

pub fn solve() -> i32 {
    0 - sorted(include_str!("../input.txt")
        .split("\n\n")
        .map(|n|
            n.lines().map(|v| 0 - v.parse::<i32>().unwrap()).sum::<i32>()
        )).take(3).sum::<i32>()
}
