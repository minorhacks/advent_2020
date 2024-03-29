use std::collections::VecDeque;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {}

#[derive(PartialEq, Eq, Debug)]
enum Token {
    Num(u64),
    Op(Op),
    OpenParen,
    CloseParen,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Op {
    Add,
    Mul,
}

pub struct Expr {
    tokens: Vec<Token>,
}

struct Context {
    acc: u64,
    op: Option<Op>,
}

struct AdvancedContext {
    acc: u64,
    op: Option<Op>,
    mul_stack: VecDeque<u64>,
}

struct NumBuilder {
    acc: Option<u64>,
}

impl Context {
    fn new() -> Context {
        Context { acc: 0, op: None }
    }
}

impl AdvancedContext {
    fn new() -> AdvancedContext {
        AdvancedContext {
            acc: 0,
            op: None,
            mul_stack: VecDeque::new(),
        }
    }
}

impl NumBuilder {
    fn new() -> NumBuilder {
        NumBuilder { acc: None }
    }

    fn push_digit(&mut self, c: char) {
        let d = c.to_digit(10).unwrap() as u64;
        match self.acc {
            None => self.acc = Some(d),
            Some(i) => self.acc = Some(10 * i + d),
        }
    }

    fn pop_reset(&mut self) -> Option<u64> {
        let temp = self.acc;
        self.acc = None;
        temp
    }
}

impl std::str::FromStr for Expr {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut tokens = Vec::new();
        let mut num_builder = NumBuilder::new();
        let mut chars_iter = s.chars();
        let mut c = chars_iter.next();
        loop {
            match c {
                None => {
                    if let Some(num) = num_builder.pop_reset() {
                        tokens.push(Token::Num(num))
                    };
                    return Ok(Expr { tokens });
                }
                Some(c) => match c {
                    '(' => tokens.push(Token::OpenParen),
                    ')' => {
                        if let Some(num) = num_builder.pop_reset() {
                            tokens.push(Token::Num(num))
                        };
                        tokens.push(Token::CloseParen);
                    }
                    '0'..='9' => num_builder.push_digit(c),
                    '+' => {
                        if let Some(num) = num_builder.pop_reset() {
                            tokens.push(Token::Num(num))
                        };
                        tokens.push(Token::Op(Op::Add));
                    }
                    '*' => {
                        if let Some(num) = num_builder.pop_reset() {
                            tokens.push(Token::Num(num))
                        };
                        tokens.push(Token::Op(Op::Mul));
                    }
                    ' ' => (),
                    _ => panic!("unexpected character: {}", c),
                },
            }
            c = chars_iter.next();
        }
    }
}

impl Expr {
    pub fn result(&self) -> u64 {
        let mut ctx = Context::new();
        let mut stack = VecDeque::new();

        for token in &self.tokens {
            match token {
                Token::Num(n) => {
                    match ctx.op {
                        None => ctx.acc = *n,
                        Some(Op::Add) => {
                            ctx.acc += n;
                            ctx.op = None;
                        }
                        Some(Op::Mul) => {
                            ctx.acc *= n;
                            ctx.op = None;
                        }
                    };
                }
                Token::Op(o) => ctx.op = Some(o.clone()),
                Token::OpenParen => {
                    stack.push_front(ctx);
                    ctx = Context::new();
                }
                Token::CloseParen => {
                    let total = ctx.acc;
                    ctx = stack.pop_front().unwrap();
                    match ctx.op {
                        None => ctx.acc = total,
                        Some(Op::Add) => ctx.acc += total,
                        Some(Op::Mul) => ctx.acc *= total,
                    };
                }
            }
        }
        ctx.acc
    }

    pub fn advanced_result(&self) -> u64 {
        let mut ctx = AdvancedContext::new();
        let mut stack = VecDeque::new();

        for token in &self.tokens {
            match token {
                Token::Num(n) => {
                    match ctx.op {
                        None => ctx.acc = *n,
                        Some(Op::Add) => {
                            ctx.acc += n;
                            ctx.op = None;
                        }
                        Some(Op::Mul) => panic!("unexpected mul op"),
                    };
                }
                Token::Op(Op::Add) => ctx.op = Some(Op::Add),
                Token::Op(Op::Mul) => {
                    ctx.mul_stack.push_front(ctx.acc);
                    ctx.acc = 0;
                    ctx.op = None;
                }

                Token::OpenParen => {
                    stack.push_front(ctx);
                    ctx = AdvancedContext::new();
                }
                Token::CloseParen => {
                    let mut total = ctx.acc;
                    while !ctx.mul_stack.is_empty() {
                        total *= ctx.mul_stack.pop_front().unwrap();
                    }
                    ctx = stack.pop_front().unwrap();
                    match ctx.op {
                        None => ctx.acc = total,
                        Some(Op::Add) => ctx.acc += total,
                        Some(Op::Mul) => ctx.mul_stack.push_front(total),
                    };
                }
            }
        }
        while !ctx.mul_stack.is_empty() {
            ctx.acc *= ctx.mul_stack.pop_front().unwrap();
        }
        ctx.acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_from_str() {
        assert_eq!(
            vec![
                Token::Num(1),
                Token::Op(Op::Add),
                Token::Num(2),
                Token::Op(Op::Mul),
                Token::Num(3),
                Token::Op(Op::Add),
                Token::Num(4),
                Token::Op(Op::Mul),
                Token::Num(5),
                Token::Op(Op::Add),
                Token::Num(6),
            ],
            "1 + 2 * 3 + 4 * 5 + 6".parse::<Expr>().unwrap().tokens
        );
        assert_eq!(
            vec![
                Token::Num(1),
                Token::Op(Op::Add),
                Token::OpenParen,
                Token::Num(234),
                Token::Op(Op::Mul),
                Token::Num(345),
                Token::CloseParen,
                Token::Op(Op::Add),
                Token::OpenParen,
                Token::Num(4),
                Token::Op(Op::Mul),
                Token::OpenParen,
                Token::Num(567),
                Token::Op(Op::Add),
                Token::Num(6),
                Token::CloseParen,
                Token::CloseParen,
            ],
            "1 + (234 * 345) + (4 * (567 + 6))"
                .parse::<Expr>()
                .unwrap()
                .tokens
        );
    }

    #[test]
    fn test_expr_result() {
        assert_eq!(
            71,
            "1 + 2 * 3 + 4 * 5 + 6".parse::<Expr>().unwrap().result()
        );
        assert_eq!(26, "2 * 3 + (4 * 5)".parse::<Expr>().unwrap().result());
        assert_eq!(
            437,
            "5 + (8 * 3 + 9 + 3 * 4 * 3)"
                .parse::<Expr>()
                .unwrap()
                .result()
        );
        assert_eq!(
            12240,
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
                .parse::<Expr>()
                .unwrap()
                .result()
        );
        assert_eq!(
            13632,
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
                .parse::<Expr>()
                .unwrap()
                .result()
        );
    }

    #[test]
    fn test_expr_advanced_result() {
        assert_eq!(
            231,
            "1 + 2 * 3 + 4 * 5 + 6"
                .parse::<Expr>()
                .unwrap()
                .advanced_result()
        );
        assert_eq!(
            46,
            "2 * 3 + (4 * 5)".parse::<Expr>().unwrap().advanced_result()
        );
        assert_eq!(
            1445,
            "5 + (8 * 3 + 9 + 3 * 4 * 3)"
                .parse::<Expr>()
                .unwrap()
                .advanced_result()
        );
        assert_eq!(
            669060,
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
                .parse::<Expr>()
                .unwrap()
                .advanced_result()
        );
        assert_eq!(
            23340,
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
                .parse::<Expr>()
                .unwrap()
                .advanced_result()
        );
    }
}
