use std::collections::{VecDeque};

const CRATE_COUNT_MAX: usize = 32;

fn solve_problem(input_data: &str) -> String {
    let mut crates: Vec<VecDeque<u8>> = vec![VecDeque::new(); CRATE_COUNT_MAX];
    let mut lines = input_data.lines();

    loop {
        let l = lines.next().unwrap();
        if l.len() == 0 { break; } else if !l.contains('[') { continue; }

        for i in 0..CRATE_COUNT_MAX {
            let left_bracket_pos = i * 4;
            if l.len() < left_bracket_pos { break; }

            if l.as_bytes()[left_bracket_pos] == b'[' {
                crates[i].push_back(l.as_bytes()[left_bracket_pos + 1]);
            }
        }
    }

    for l in lines {
        let move_instruction = l.split(' ').into_iter().collect::<Vec<&str>>();
        let count = move_instruction[1].parse().unwrap();
        let src = move_instruction[3].parse::<usize>().unwrap() - 1;
        let dst = move_instruction[5].parse::<usize>().unwrap() - 1;

        for _ in 0..count {
            let c = crates[src].pop_front().unwrap();
            crates[dst].push_front(c);
        }
    }

    let mut ret = String::new();
    for c in crates {
        if c.len() != 0 {
            ret.push(*c.front().unwrap() as char);
        }
    }
    ret
}

pub fn solve() -> String {
    solve_problem(include_str!("../input.txt"))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve_problem(include_str!("../input_test.txt"));
        assert_eq!(result, "CMZ");
    }
}
