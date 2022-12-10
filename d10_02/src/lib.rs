const PIXEL_COLS: usize = 40;
const PIXEL_ROWS: usize = 6;

fn solve_problem(input_data: &str) -> i32 {
    let lines = input_data.lines();
    let mut x_reg: i32 = 1;
    let mut cycle_count: i32 = 0;

    let mut display = [0u8; PIXEL_ROWS * PIXEL_COLS];

    for l in lines {
        let mut cmd = l.split(' ');
        let mut display_check = || {
            cycle_count += 1;
            let screen_pos = cycle_count - 1;
            if screen_pos % PIXEL_COLS as i32 >= x_reg - 1 && screen_pos % PIXEL_COLS as i32 <= x_reg + 1 {
                display[screen_pos as usize] = 1;
            }
        };


        match cmd.next().unwrap() {
            "noop" => {
                display_check();
            }
            "addx" => {
                display_check();
                display_check();
                x_reg += cmd.next().unwrap().parse::<i32>().unwrap();
            }
            _ => {}
        }
    }

    display.into_iter().enumerate().for_each(|(pos, val)| {
        if pos % PIXEL_COLS == 0 { println!(); }
        print!("{}", if val != 0 {"#"} else {" "} );
    });
    0
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
        assert_eq!(result, 0);
    }
}
