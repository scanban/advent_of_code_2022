use itertools::Itertools;

const SIGNAL_BUFFER_LEN: usize = 14;

fn solve_problem(input_data: &str) -> i32 {
    let signal = input_data.as_bytes();

    for signal_pos in SIGNAL_BUFFER_LEN..signal.len() {
        if signal[signal_pos - SIGNAL_BUFFER_LEN..signal_pos].iter().all_unique() { return (signal_pos) as i32; }
    }
    unreachable!()
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
        assert_eq!(result, 19);
    }
}
