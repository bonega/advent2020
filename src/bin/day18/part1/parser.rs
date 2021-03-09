use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    character::complete::{char, one_of},
    IResult,
    multi::fold_many0,
    sequence::delimited,
};
use nom::character::complete::{digit1, multispace0};
use nom::combinator::all_consuming;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Number(usize),
    Add { op1: Box<Expr>, op2: Box<Expr> },
    Mul { op1: Box<Expr>, op2: Box<Expr> },
}

pub fn parse(s: &str) -> Result<usize> {
    let (_, expr) = all_consuming(expr)(s).map_err(|_| anyhow!("Parse failed for '{}'", s))?;
    Ok(expr.as_number())
}

impl Expr {
    pub fn as_number(&self) -> usize {
        match self {
            Expr::Number(x) => *x,
            Expr::Add { op1, op2 } => op1.as_number() + op2.as_number(),
            Expr::Mul { op1, op2 } => op1.as_number() * op2.as_number(),
        }
    }
}

#[allow(dead_code)]
pub fn expr(input: &str) -> IResult<&str, Expr> {
    // let (input, op1) = expr1(input)?;
    let (input, op1) = expr2(input)?;
    fold_many0(expr_second_arm, op1, |e1, (op, op2)| match op {
        '+' => Expr::Add {
            op1: e1.into(),
            op2: op2.into(),
        },
        '*' => Expr::Mul {
            op1: e1.into(),
            op2: op2.into(),
        },
        _ => panic!(),
    })(input)
}

fn expr_second_arm(input: &str) -> IResult<&str, (char, Expr)> {
    let (input, op) = one_of("+*")(input)?;
    let (input, op2) = expr2(input)?;
    Ok((input, (op, op2)))
}

fn expr2(input: &str) -> IResult<&str, Expr> {
    delimited(multispace0, alt((expr2_number
                                , expr2_paren)), multispace0)(input)
}

fn expr2_number(input: &str) -> IResult<&str, Expr> {
    let (input, number) = digit1(input)?;
    Ok((input, Expr::Number(number.parse().unwrap())))
}

fn expr2_paren(input: &str) -> IResult<&str, Expr> {
    delimited(char('('), expr, char(')'))(input)
}

#[test]
fn testing() {
    let s1 = "1+(2*3+1)+2";
    let s1_ws = "1+ ( 2*3+1)+2";
    let (_, exp1) = expr(s1).unwrap();
    let (_, exp1_ws) = expr(s1_ws).unwrap();
    assert!(is_match(s1_ws));
    assert_eq!(exp1, exp1_ws);
    assert_eq!(10, exp1.as_number());
}

fn is_match(s: &str) -> bool {
    match expr(s) {
        Ok(("", _)) => true,
        _ => false
    }
}