pub fn solve() -> i32 {
    include_str!("../input.txt")
        .split("\n\n")
        .map(|n|
            n.lines().map(|v| v.parse::<i32>().unwrap()).sum::<i32>()
        ).max().unwrap()
}
