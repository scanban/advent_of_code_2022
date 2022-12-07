const MAXIMUM_DIRECTORY_SIZE: usize = 100_000;

fn calculate_dir(total_size: &mut usize, lines: &mut dyn Iterator<Item=&str>) -> usize {
    let mut sz: usize = 0;

    loop {
        match lines.next() {
            Some(l) => {
                if l.starts_with("$ cd") {
                    let dir = l.split(' ').skip(2).next().unwrap();
                    if dir.eq("..") {
                        break;
                    } else {
                        sz += calculate_dir(total_size, lines);
                    }
                } else if !l.starts_with("dir") && !l.starts_with("$ ls") {
                    sz += l.split(' ').next().unwrap().parse::<usize>().unwrap();
                }
            }
            _ => {
                break;
            }
        }
    }
    if sz < MAXIMUM_DIRECTORY_SIZE {
        *total_size += sz;
    }
    sz
}

fn solve_problem(input_data: &str) -> usize {
    let mut result = 0usize;

    println!("total occupied size: {}", calculate_dir(&mut result,
                                                      &mut input_data.lines().into_iter()));
    result
}

pub fn solve() -> usize {
    solve_problem(include_str!("../input.txt"))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve_problem(include_str!("../input_test.txt"));
        assert_eq!(result, 95437);
    }
}
