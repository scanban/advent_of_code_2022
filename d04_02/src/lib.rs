use std::collections::HashSet;


fn solve_problem(input_data: &str) -> i32 {
    let mut lines = input_data.lines();
    let mut result: i32 = 0;

    for l in lines {
        let epair = l.split(',').collect::<Vec<&str>>();
        let e1 = epair[0].split("-").map(|v| v.parse().unwrap()).collect::<Vec<i32>>();
        let e2 = epair[1].split("-").map(|v| v.parse().unwrap()).collect::<Vec<i32>>();

        if e1[0] <= e2[0] && e2[0] <= e1[1] {
            result += 1
        } else if e2[0] <= e1[0] && e1[0] <= e2[1] {
            result += 1
        }
    }
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
        assert_eq!(result, 4);
    }
}
