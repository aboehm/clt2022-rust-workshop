use std::convert::From;
use std::fmt;

#[derive(Debug)]
enum OperationType {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
struct Operation<T> {
    a: T,
    b: T,
    op: OperationType,
}

impl<T> fmt::Display for Operation<T> 
where
    T: std::ops::Add<Output=T>,
    T: std::ops::Sub<Output=T>,
    T: std::ops::Mul<Output=T>,
    T: std::ops::Div<Output=T>,
    T: std::marker::Copy,
    T: std::fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let res = match self.op {
            OperationType::Add => self.a + self.b,
            OperationType::Sub => self.a - self.b,
            OperationType::Mul => self.a * self.b,
            OperationType::Div => self.a / self.b,
        };
        write!(formatter, "{}", res)
    }
}

impl From<&str> for OperationType {
    fn from(input: &str) -> Self {
        match input {
            "+" => OperationType::Add,
            "-" => OperationType::Sub,
            "*" => OperationType::Mul,
            "/" => OperationType::Div,
            _   => panic!("Not implemented"),
        }
    }
}

fn main() -> Result<(), String> {
    let a = 1;
    let t = "*";
    let b = 2;
    let op = Operation { a: a, op: t.into(), b: b};
    println!("Result: {}", op);
    Ok(())
}
