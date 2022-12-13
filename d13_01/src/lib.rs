use std::cmp::max;

#[derive(Debug)]
enum Element {
    INTEGER(i32),
    LIST(Vec<Element>),
}


fn parse_stage(chars: &mut Vec<u8>) -> Option<Element> {
    while !chars.is_empty() {
        let c = chars.pop().unwrap();
        match c {
            b'0'..=b'9' => {
                let mut s = String::new();
                s.push(c as char);
                while (b'0'..=b'9').contains(chars.last().unwrap()) {
                    s.push(chars.pop().unwrap() as char);
                }
                return Some(Element::INTEGER(s.parse().unwrap()));
            }
            b',' => {}
            b'[' => {
                let mut ret = Vec::<Element>::new();
                while chars.last().unwrap() != &b']' {
                    ret.push(parse_stage(chars).unwrap());
                }
                chars.pop();
                return Some(Element::LIST(ret));
            }
            _ => unreachable!(),
        }
    }
    None
}

#[derive(Debug)]
enum Result { Right, Wrong, Equal }

fn compare_messages(lhs: &Element, rhs: &Element) -> Result {
    match lhs {
        Element::INTEGER(l) => return match rhs {
            Element::INTEGER(r) =>
                if l > r { Result::Wrong } else if l < r { Result::Right } else { Result::Equal },
            Element::LIST(_) =>
                compare_messages(&Element::LIST(Vec::from([Element::INTEGER(*l)])), rhs),
        },
        Element::LIST(l) => {
            match rhs {
                Element::INTEGER(r) =>
                    return compare_messages(lhs, &Element::LIST(Vec::from([Element::INTEGER(*r)]))),
                Element::LIST(r) => {
                    for i in 0..=max(l.len(), r.len()) {
                        if l.len() == i && r.len() == i { return Result::Equal; }
                        if l.len() == i { return Result::Right; }
                        if r.len() == i { return Result::Wrong; }
                        let c = compare_messages(&l[i], &r[i]);
                        if matches!(c,Result::Equal) { continue; }
                        return c;
                    }
                }
            }
        }
    }
    Result::Equal
}

fn solve_problem(input_data: &str) -> i32 {
    let mut result = 0;
    let mut idx = 1;

    for block in input_data.split("\n\n") {
        let mut lines = block.lines();
        let lhs = parse_stage(&mut lines.next().unwrap().as_bytes().iter().cloned().rev().collect::<Vec<u8>>());
        let rhs = parse_stage(&mut lines.next().unwrap().as_bytes().iter().cloned().rev().collect::<Vec<u8>>());
        if !matches!(compare_messages(lhs.as_ref().unwrap(), rhs.as_ref().unwrap()), Result::Wrong) {
            result += idx;
        }
        idx += 1;
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
        assert_eq!(result, 13);
    }
}
