use std::collections::HashSet;

fn solve_problem(input_data: &str) -> i32 {
    let v = input_data
        .lines().map(|l| {
        let mut chunks = l.as_bytes().chunks(l.len() / 2);
        let lhs: HashSet<u8> = HashSet::from_iter(chunks.next().unwrap().iter().map(|v| *v));
        let rhs: HashSet<u8> = HashSet::from_iter(chunks.next().unwrap().iter().map(|v| *v));
        lhs.intersection(&rhs).map(|v|
            match v {
                b'a'..=b'z' => (v - b'a') as i32 + 1,
                b'A'..=b'Z' => (v - b'A') as i32 + 27,
                _ => unreachable!(),
            }).sum::<i32>()
    }).sum();
    v
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
        assert_eq!(result, 157);
    }
}
