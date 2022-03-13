use std::io;
use std::convert::From;

enum Operation {
    Invalid(String),
    Add,
    Sub,
    Mul,
    Div,
}

impl From<&str> for Operation {
    fn from(string: &str) -> Self {
        match string {
            "+" => Operation::Add,
            "-" => Operation::Sub,
            "*" => Operation::Mul,
            "/" => Operation::Div,
            _ => Operation::Invalid(string.to_string()),
        }
    }
}

fn operate(a: i32, op: Operation, b: i32) -> Result<i32, String> {
    match op {
        Operation::Div => match b {
            0 => Err("Division by Zero".to_string()),
            _ => Ok(a/b),
        },
        Operation::Add => Ok(a+b),
        Operation::Sub => Ok(a-b),
        Operation::Mul => Ok(a*b),
        Operation::Invalid(string) =>
            Err(format!("{} is not a operation", string)),
    }
}

fn read_integer_from_stdin() -> Result<i32, &'static str> {
    println!("Value? ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().map_err(|_| "Not a number")
}

fn read_op_from_stdin() -> Result<Operation, &'static str> {
    println!("Operation? ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    Ok(input.trim().into())
}

fn main() -> Result<(), String> {
    let a = read_integer_from_stdin()?;
    let op = read_op_from_stdin()?;
    let b = read_integer_from_stdin()?;
    println!("Result: {}", operate(a, op, b).expect("A result"));
    Ok(())
}
