#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Sum,
    Product,
    Value(isize),
}

fn try_insert_operation(
    stack: &mut Vec<char>,
    keep_par: bool,
    op: char,
    respect_order: bool,
) -> Vec<Expression> {
    let mut result = vec![];
    while let Some(e) = stack.pop() {
        if e == '(' {
            if keep_par {
                stack.push(e);
            }
            break;
        } else if op != ')' && respect_order && !(op == '*' && e == '+') {
            stack.push(e);
            break;
        }
        result.push(Expression::from(e).unwrap());
    }
    result
}

impl Expression {
    fn from(input: char) -> Option<Expression> {
        match input {
            '*' => Some(Expression::Product),
            '+' => Some(Expression::Sum),
            _ => None,
        }
    }

    fn parse(input: &str, respect_order: bool) -> Vec<Expression> {
        let mut input = input.to_string();
        input.push(' ');
        let mut output: Vec<Expression> = vec![];
        let mut exp: Vec<char> = vec![];
        let mut chars = input.chars();
        let mut next: Option<char> = chars.next();

        loop {
            if next == None {
                break;
            }
            let mut move_next = true;
            let c = next.unwrap_or(' ');
            match c {
                '0'..='9' => {
                    move_next = false;
                    let mut buff: Vec<u8> = vec![c as u8];
                    'outer: while let Some(n) = chars.next() {
                        match n {
                            '0'..='9' => {
                                buff.push(n as u8);
                            }
                            _ => {
                                next = Some(n);
                                break 'outer;
                            }
                        }
                    }
                    let value: isize = String::from_utf8(buff.drain(0..).collect())
                        .map(|s| s.parse::<isize>().unwrap_or(0))
                        .unwrap();
                    output.push(Expression::Value(value))
                }
                '+' | '*' => {
                    output.extend(try_insert_operation(&mut exp, true, c, respect_order));
                    exp.push(c);
                }
                '(' => {
                    exp.push('(');
                }
                ')' => {
                    output.extend(try_insert_operation(&mut exp, false, ')', respect_order));
                }
                _ => {}
            }
            if move_next {
                next = chars.next();
            }
        }
        while let Some(e) = exp.pop() {
            output.push(Expression::from(e).unwrap());
        }
        output
    }
}

fn solve(exp: &[Expression]) -> Option<isize> {
    exp.iter()
        .fold(vec![], |mut stack, e| {
            match *e {
                Expression::Value(value) => stack.push(value),
                Expression::Sum => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a + b);
                }
                Expression::Product => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a * b);
                }
            }
            stack
        })
        .pop()
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &str) -> isize {
    let input: Vec<Vec<Expression>> = input
        .lines()
        .map(|line| Expression::parse(line, false))
        .collect();
    input.iter().flat_map(|e| solve(e)).sum()
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &str) -> isize {
    let input: Vec<Vec<Expression>> = input
        .lines()
        .map(|line| Expression::parse(line, true))
        .collect();
    input.iter().flat_map(|e| solve(e)).sum()
}
