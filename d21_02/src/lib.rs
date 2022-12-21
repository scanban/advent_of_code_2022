#![allow(dead_code, unused_mut, unused_variables, non_camel_case_types)]

use std::collections::BTreeMap;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
enum MOperation {
    YELL_NUMBER { lhs: String, value: i64 },
    OP_PLUS { lhs: String, rhs: String },
    OP_MINUS { lhs: String, rhs: String },
    OP_MUL { lhs: String, rhs: String },
    OP_DIV { lhs: String, rhs: String },
    OP_EQ { lhs: String, rhs: String },
}

#[derive(Debug, Clone)]
enum COperations {
    RESOLVE(String),
    OP_PLUS,
    OP_MINUS,
    OP_DIV,
    OP_MUL,
    OP_EQ,
}

const HUMN_VARIABLE_NAME: &str = "humn";

fn solve_problem(input_data: &str) -> i64 {
    lazy_static! {
        static ref RE_M_NUMBER: Regex = Regex::new(r"^(\w+): (\d+)$").unwrap();
        static ref RE_M_OP: Regex = Regex::new(r"^(\w+): (\w+) (\+|-|\*|/) (\w+)$").unwrap();
    }
    let lines = input_data.lines();

    let monkeys = lines
        .map(|v| {
            if RE_M_NUMBER.is_match(v) {
                let c = RE_M_NUMBER.captures(v).unwrap();
                let lhs = c.get(1).unwrap().as_str().to_string();
                (lhs.to_string(), (MOperation::YELL_NUMBER { lhs: lhs.to_string(), value: c.get(2).unwrap().as_str().parse::<i64>().unwrap() }))
            } else {
                let c = RE_M_OP.captures(v).unwrap();
                let name = c.get(1).unwrap().as_str().to_string();
                let lhs = c.get(2).unwrap().as_str();
                let op = c.get(3).unwrap().as_str();
                let rhs = c.get(4).unwrap().as_str();
                match op {
                    "+" => (
                        name,
                        (MOperation::OP_PLUS {
                            lhs: lhs.to_string(),
                            rhs: rhs.to_string(),
                        }),
                    ),
                    "-" => (
                        name,
                        (MOperation::OP_MINUS {
                            lhs: lhs.to_string(),
                            rhs: rhs.to_string(),
                        }),
                    ),
                    "*" => (
                        name,
                        (MOperation::OP_MUL {
                            lhs: lhs.to_string(),
                            rhs: rhs.to_string(),
                        }),
                    ),
                    "/" => (
                        name,
                        (MOperation::OP_DIV {
                            lhs: lhs.to_string(),
                            rhs: rhs.to_string(),
                        }),
                    ),
                    _ => unreachable!(),
                }
            }
        })
        .collect::<BTreeMap<String, MOperation>>();

    //dbg!(&monkeys);
    let mut operations_stack = Vec::<COperations>::new();
    let mut operand_stack = Vec::<(String, i64)>::new();
    let mut humn_op_stack = Vec::<(u8, i64)>::new();

    let mut process_operation = |op: &MOperation,
                                 operations_stack: &mut Vec<COperations>, operand_stack: &mut Vec<(String, i64)>| {
        match op {
            MOperation::OP_PLUS { lhs, rhs } => {
                operations_stack.push(COperations::OP_PLUS);
                operations_stack.push(COperations::RESOLVE(lhs.clone()));
                operations_stack.push(COperations::RESOLVE(rhs.clone()));
            }
            MOperation::OP_MINUS { lhs, rhs } => {
                operations_stack.push(COperations::OP_MINUS);
                operations_stack.push(COperations::RESOLVE(lhs.clone()));
                operations_stack.push(COperations::RESOLVE(rhs.clone()));
            }
            MOperation::OP_MUL { lhs, rhs } => {
                operations_stack.push(COperations::OP_MUL);
                operations_stack.push(COperations::RESOLVE(lhs.clone()));
                operations_stack.push(COperations::RESOLVE(rhs.clone()));
            }
            MOperation::OP_DIV { lhs, rhs } => {
                operations_stack.push(COperations::OP_DIV);
                operations_stack.push(COperations::RESOLVE(lhs.clone()));
                operations_stack.push(COperations::RESOLVE(rhs.clone()));
            }
            MOperation::OP_EQ { lhs, rhs } => {
                operations_stack.push(COperations::OP_EQ);
                operations_stack.push(COperations::RESOLVE(lhs.clone()));
                operations_stack.push(COperations::RESOLVE(rhs.clone()));
            }
            MOperation::YELL_NUMBER { lhs, value } => operand_stack.push((lhs.to_string(), *value as i64)),
        };
    };

    let new_root_op = match monkeys.get("root").unwrap() {
        MOperation::OP_PLUS { lhs, rhs } =>
            MOperation::OP_EQ { lhs: lhs.clone(), rhs: rhs.clone() },
        MOperation::OP_MINUS { lhs, rhs } =>
            MOperation::OP_EQ { lhs: lhs.clone(), rhs: rhs.clone() },
        MOperation::OP_MUL { lhs, rhs } =>
            MOperation::OP_EQ { lhs: lhs.clone(), rhs: rhs.clone() },
        MOperation::OP_DIV { lhs, rhs } =>
            MOperation::OP_EQ { lhs: lhs.clone(), rhs: rhs.clone() },
        _ => unreachable!(),
    };
    process_operation(&new_root_op, &mut operations_stack, &mut operand_stack);

    while !operations_stack.is_empty() {
        let e = operations_stack.pop().unwrap();
        let mut kvv = |op, lhs:&(String, i64), rhs:&(String, i64)| {
            if lhs.0.as_str().eq(HUMN_VARIABLE_NAME) || rhs.0.as_str().eq(HUMN_VARIABLE_NAME) {
                if lhs.0.as_str().eq(HUMN_VARIABLE_NAME) {
                    humn_op_stack.push((op, rhs.1));
                } else {
                    match op {
                        b'+' | b'*' => humn_op_stack.push((op, lhs.1)),
                        b'-' => {
                            humn_op_stack.push((b'*', -1));
                            humn_op_stack.push((b'+', lhs.1));
                        },
                        _  => {
                            unreachable!()
                        }
                    }
                }
                HUMN_VARIABLE_NAME.to_string()
            }
            else {"###".to_string()}
        };

        match e {
            COperations::RESOLVE(s) => {
                process_operation(monkeys.get(&s).unwrap(), &mut operations_stack, &mut operand_stack);
            }
            COperations::OP_PLUS => {
                let lhs = operand_stack.pop().unwrap();
                let rhs = operand_stack.pop().unwrap();
                if cfg!(test) {
                    println!("{}:[{}] + {}:[{}]", lhs.0, lhs.1, rhs.0, rhs.1);
                }
                operand_stack.push((kvv(b'+', &lhs, &rhs), lhs.1 + rhs.1));
            }
            COperations::OP_MINUS => {
                let lhs = operand_stack.pop().unwrap();
                let rhs = operand_stack.pop().unwrap();
                if cfg!(test) {
                    println!("{}:[{}] - {}:[{}]", lhs.0, lhs.1, rhs.0, rhs.1);
                }
                operand_stack.push((kvv(b'-', &lhs, &rhs), lhs.1 - rhs.1));
            }
            COperations::OP_MUL => {
                let lhs = operand_stack.pop().unwrap();
                let rhs = operand_stack.pop().unwrap();
                if cfg!(test) {
                    println!("{}:[{}] * {}:[{}]", lhs.0, lhs.1, rhs.0, rhs.1);
                }
                operand_stack.push((kvv(b'*', &lhs, &rhs), lhs.1 * rhs.1));
            }
            COperations::OP_DIV => {
                let lhs = operand_stack.pop().unwrap();
                let rhs = operand_stack.pop().unwrap();
                if cfg!(test) {
                    println!("{}:[{}] / {}:[{}]", lhs.0, lhs.1, rhs.0, rhs.1);
                }
                operand_stack.push((kvv(b'/', &lhs, &rhs), lhs.1 / rhs.1));
            }
            COperations::OP_EQ => {
                let lhs = operand_stack.pop().unwrap();
                let rhs = operand_stack.pop().unwrap();
                if cfg!(test) {
                    println!("lhs: {}, rhs:{}", lhs.1, rhs.1);
                }
                operand_stack.push(("###".to_string(), rhs.1 as i64));
            }
        }
    }

    let mut humn_value = operand_stack.pop().unwrap().1;

    for (op, val) in humn_op_stack.iter().rev() {
        match *op {
            b'+' => humn_value -= *val,
            b'-' => humn_value += *val,
            b'*' => humn_value /= val,
            b'/' => humn_value *= val,
            _ => unreachable!(),
        }
    }

    humn_value
}

pub fn solve() -> i64 {
    solve_problem(include_str!("../input.txt"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve_problem(include_str!("../input_test.txt"));
        assert_eq!(result, 301);
    }
}
