use std::collections::HashSet;

fn solve_problem(input_data: &str) -> i32 {
    input_data.lines().map(|l| {
        let w: Vec<i32> = l.split(&[',', '-']).map(|v| v.parse().unwrap()).collect();
        if w[0] <= w[2] && w[2] <= w[1] {
            1
        } else if w[2] <= w[0] && w[0] <= w[3] {
            1
        } else { 0 }
    }).sum()
}

pub fn solve() -> i32 {
    solve_problem(include_str!("../input.txt"))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve_problem(include_str!("../input_test.txt"));
        assert_eq!(result, 4);
    }
}
