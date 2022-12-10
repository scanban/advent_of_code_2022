const TARGET_CYCLE_INCREMENT: i32 = 40;

fn solve_problem(input_data: &str) -> i32 {
    let lines = input_data.lines();

    let mut x_reg = 1;
    let mut cycle_count = 0;
    let mut cycle_target = 20;
    let mut total_signal_strength = 0;

    for l in lines {
        let mut cmd = l.split(' ');
        let mut cycle_check = || {
            cycle_count += 1;
            if cycle_count == cycle_target {
                if cfg!(test) {
                    println!("cc: {}, x: {}, strength: {}", cycle_count, x_reg, cycle_count * x_reg);
                }
                total_signal_strength += cycle_count * x_reg;
                cycle_target += TARGET_CYCLE_INCREMENT;
            }
        };

        match cmd.next().unwrap() {
            "noop" => {
                cycle_check();
            }
            "addx" => {
                cycle_check();
                cycle_check();
                x_reg += cmd.next().unwrap().parse::<i32>().unwrap();
            }
            _ => {}
        }
    }

    total_signal_strength
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
        assert_eq!(result, 13140);
    }
}
