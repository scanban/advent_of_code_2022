use std::collections::{BTreeSet, VecDeque};
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug)]
enum Operand {
    OLD,
    NUMBER(i32),
}

impl Operand {
    fn new(v: &str) -> Operand {
        if v.eq("old") { return Operand::OLD; }
        Operand::NUMBER(v.parse().unwrap())
    }
}

#[derive(Debug)]
struct Operation {
    lhs: Operand,
    rhs: Operand,
    operation: u8,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    operation: Operation,
    t_divisible: i32,
    target_true: i32,
    target_false: i32,
    inspected_items: usize,
}

impl Monkey {
    fn process(&mut self, common_divisor: i64) -> Vec<(usize, i64)> {
        let mut ret = Vec::<(usize, i64)>::new();

        for v in &self.items {
            let lhs = match self.operation.lhs {
                Operand::OLD => { *v }
                Operand::NUMBER(i) => { i as i64 }
            };
            let rhs = match self.operation.rhs {
                Operand::OLD => { *v }
                Operand::NUMBER(i) => { i as i64 }
            };

            let mut worry_level = if self.operation.operation == b'+' { lhs + rhs } else { lhs * rhs };
            worry_level %= common_divisor;

            if worry_level % (self.t_divisible as i64) == 0 {
                ret.push((self.target_true as usize, worry_level));
            } else {
                ret.push((self.target_false as usize, worry_level));
            }
        }
        self.inspected_items += self.items.len();
        self.items.clear();

        ret
    }
}

fn solve_problem(input_data: &str, rounds: usize) -> usize {
    lazy_static! {
        static ref RE_ITEMS: Regex = Regex::new(r"^\s+Starting items: ((?:\d+, )*\d+)$").unwrap();
        static ref RE_OPERATION: Regex = Regex::new(r"^\s+Operation: new = (old|\d+) (\+|\*) (old|\d+)$").unwrap();
        static ref RE_DIVISIBLE: Regex = Regex::new(r"^\s+Test: divisible by (\d+)$").unwrap();
        static ref RE_IF: Regex = Regex::new(r"^\s+If (?:true|false): throw to monkey (\d+)$").unwrap();
    }

    let mut monkeys = input_data.split("\n\n").map(|s| {
        let mut lines = s.lines().skip(1);
        let items_str = RE_ITEMS.captures(lines.next().unwrap()).unwrap().get(1).unwrap().as_str();
        let operation_caps = RE_OPERATION.captures(lines.next().unwrap()).unwrap();

        Monkey {
            items: items_str.split(", ").map(|v| v.parse().unwrap()).collect::<VecDeque<i64>>(),
            operation: Operation {
                lhs: Operand::new(operation_caps.get(1).unwrap().as_str()),
                operation: operation_caps.get(2).unwrap().as_str().as_bytes()[0],
                rhs: Operand::new(operation_caps.get(3).unwrap().as_str()),
            },
            t_divisible: RE_DIVISIBLE.captures(lines.next().unwrap()).unwrap().get(1).unwrap().as_str().parse().unwrap(),
            target_true: RE_IF.captures(lines.next().unwrap()).unwrap().get(1).unwrap().as_str().parse().unwrap(),
            target_false: RE_IF.captures(lines.next().unwrap()).unwrap().get(1).unwrap().as_str().parse().unwrap(),
            inspected_items: 0,
        }
    }).collect::<Vec<Monkey>>();

    let common_divisor = monkeys.iter().map(|v| v.t_divisible as i64).product();

    for _ in 0..rounds {
        for monkey_id in 0..monkeys.len() {
            let process_list = monkeys[monkey_id].process(common_divisor);
            process_list.into_iter().for_each(|(m_id, w_level)| {
                monkeys[m_id].items.push_back(w_level);
            })
        }
    }

    let inspections = monkeys.iter().map(|v| v.inspected_items).collect::<BTreeSet<usize>>();
    inspections.into_iter().rev().take(2).product::<usize>()
}

pub fn solve() -> usize {
    solve_problem(include_str!("../input.txt"), 10000)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve_problem(include_str!("../input_test.txt"), 10000);
        assert_eq!(result, 2713310158);
    }
}
