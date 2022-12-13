use std::cmp::{max, Ordering};

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

#[derive(Debug)]
struct Packet<'a> {
    s: &'a str,
    e: Element,
}

impl PartialOrd for Packet<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let result = compare_messages(&self.e, &other.e);
        match result {
            Result::Right => Some(Ordering::Less),
            Result::Equal => Some(Ordering::Equal),
            Result::Wrong => Some(Ordering::Greater),
        }
    }
}

impl PartialEq for Packet<'_> {
    fn eq(&self, other: &Self) -> bool {
        matches!(compare_messages(&self.e, &other.e), Result::Equal)
    }
}

fn solve_problem(input_data: &str) -> i32 {
    const DIVIDER_PACKETS: [&str; 2] = ["[[2]]", "[[6]]"];
    let mut result = 1;
    let mut v = Vec::<Packet>::new();

    for block in input_data.split("\n\n") {
        let mut lines = block.lines();
        let l_lhs = lines.next().unwrap();
        let l_rhs = lines.next().unwrap();
        let lhs = parse_stage(&mut l_lhs.as_bytes().iter().cloned().rev().collect::<Vec<u8>>());
        let rhs = parse_stage(&mut l_rhs.as_bytes().iter().cloned().rev().collect::<Vec<u8>>());
        v.push(Packet { s: l_lhs, e: lhs.unwrap() });
        v.push(Packet { s: l_rhs, e: rhs.unwrap() });
    }

    v.extend(DIVIDER_PACKETS.into_iter().map(|v|{
        Packet{ s: v, e: parse_stage(&mut v.as_bytes().iter().cloned().rev().collect::<Vec<u8>>()).unwrap() }
    }));

    v.sort_by(|a, b| a.partial_cmp(b).unwrap());

    for i in 0..v.len() {
        if v[i].s.eq(&"[[2]]".to_string()) { result *= i as i32 + 1; }
        if v[i].s.eq(&"[[6]]".to_string()) { result *= i as i32 + 1; }
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
        assert_eq!(result, 140);
    }
}
