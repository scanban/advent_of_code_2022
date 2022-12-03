fn solve_problem(input_data: &str) -> i32 {
    include_str!("../input.txt");
    0
}

pub fn solve() -> i32 {
    solve_problem("../input.txt")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve_problem("../input_test.txt");
        assert_eq!(result, 4);
    }
}
