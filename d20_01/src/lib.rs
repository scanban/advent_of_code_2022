#[derive(Debug, Copy, Clone)]
struct NumNode {
    pos: usize,
    val: i32,
}

fn solve_problem(input_data: &str) -> i32 {
    let lines = input_data.lines();

    let mut work_data = lines.enumerate().map(|(k, s)|
        NumNode { pos: k, val: s.parse().unwrap() }).collect::<Vec<NumNode>>();

    if cfg!(test) {
        dbg!(&work_data);
    }

    for i in 0..work_data.len() {
        let position = work_data.iter().position(|&d| d.pos == i).unwrap() as i32;
        let v = *&work_data[position as usize].val;
        if v != 0 {
            if v >= 0 {
                let mut s = position;
                let mut e = position + 1;

                for _ in 0..v % (work_data.len() - 1) as i32 {
                    if s >= work_data.len() as i32 { s -= work_data.len() as i32; }
                    if e >= work_data.len() as i32 { e -= work_data.len() as i32; }
                    work_data.swap(s as usize, e as usize);
                    s += 1;
                    e += 1;
                }
            } else {
                if cfg!(test) {
                    println!("v: {}, {}:{}", v, position, position + v % work_data.len() as i32);
                }

                let mut s = position;
                let mut e = position - 1;
                if cfg!(test) {
                    println!("s:{}, e:{}", s, e);
                }
                for _ in 0..v.abs() % (work_data.len() - 1) as i32 {
                    if s < 0 { s += work_data.len() as i32; }
                    if e < 0 { e += work_data.len() as i32; }
                    work_data.swap(s as usize, e as usize);
                    s -= 1;
                    e -= 1;
                }
                if cfg!(test) {
                    println!("s:{}, e:{}", s, e);
                    println!("v: {}, original position: {}, new position {}, iter {}",
                             v, position, work_data.iter().position(|&d| d.pos == position as usize).unwrap(),
                             v.abs() % work_data.len() as i32);
                }
            }
        }

        if cfg!(test) {
            println!("{:3} -> {:?}", v, work_data);
        }
    }

    let np = work_data.iter().position(|&d| d.val == 0).unwrap();
    work_data[(np + 1000) % work_data.len()].val
        + work_data[(np + 2000) % work_data.len()].val + work_data[(np + 3000) % work_data.len()].val
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
        assert_eq!(result, 3);
    }
}
