use std::collections::HashSet;

fn solve_problem(input_data: &str) -> i32 {
    let mut lines = input_data.lines().peekable();
    let mut result = 0;
    loop {
        let r1:HashSet<&u8> = HashSet::from_iter(lines.next().unwrap().as_bytes());
        let r2:HashSet<&u8> = HashSet::from_iter(lines.next().unwrap().as_bytes());
        let r3:HashSet<&u8> = HashSet::from_iter(lines.next().unwrap().as_bytes());

        result += r3.intersection(&(r1.intersection(&r2).cloned().collect())).map(|v|
            match v {
                b'a'..=b'z' => (*v - b'a') as i32 + 1,
                b'A'..=b'Z' => (*v - b'A') as i32 + 27,
                _ => unreachable!(),
            }).sum::<i32>();
        let _ = lines.peek().is_some() || break;
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
        assert_eq!(result, 70);
    }
}
