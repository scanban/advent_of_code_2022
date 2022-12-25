#![allow(dead_code, unused_mut, unused_variables)]

fn snafu2decimal(snafu: &[u8]) -> i64 {
    snafu.iter().rev().enumerate().map(|(n, v)| {
        let place = i64::pow(5, n as u32);
        match *v {
            b'2' => place * 2,
            b'1' => place,
            b'0' => 0,
            b'-' => -place,
            b'=' => -2 * place,
            _ => unreachable!()
        }
    }).sum()
}

fn decimal2snafu(dec: i64) -> String {
    const DIGITS: [u8; 5] = [b'0', b'1', b'2', b'=', b'-'];
    let mut buf = Vec::<u8>::new();
    let mut source = dec;
    loop {
        let digit = source % 5;
        buf.push(DIGITS[digit as usize]);
        if digit > 2 { source = source / 5 + 1 }
        else {source = source / 5 }
        if source == 0 { break; }
    }

    buf.reverse();
    String::from_utf8(buf).unwrap()
}

fn solve_problem(input_data: &str) -> String {
    let lines = input_data.lines();

    let sum = lines.map(|v| snafu2decimal(v.as_bytes())).sum::<i64>();
    decimal2snafu(sum)
}

pub fn solve() -> String {
    solve_problem(include_str!("../input.txt"))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        assert_eq!("2=-1=0".to_string(), solve_problem(include_str!("../input_test.txt")));
    }
}
