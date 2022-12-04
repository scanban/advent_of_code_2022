use std::collections::HashSet;

fn solve_problem(input_data: &str) -> i32 {
    let mut lines = input_data.lines().peekable();
    let mut result = 0;
    result
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
        assert_eq!(result, 70);
    }
}
