use strum_macros::EnumIter;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Debug, EnumIter)]
pub enum Builtin {
    Add,
    Sub,
    Mul,
    Div,
    Drop,
    Dup,
    Swap,
    Over
}

pub enum BuiltinError { InvalidBuiltin }

impl Builtin {
    pub fn call(self, stack: &mut Vec<i32>) -> Result<(), Error> {
        let size = stack.len();
        match self {
            Builtin::Add => binary_math(stack, |a,b| { Ok(a + b) }),
            Builtin::Sub => binary_math(stack, |a,b| { Ok(a - b) }),
            Builtin::Mul => binary_math(stack, |a,b| { Ok(a * b) }),
            Builtin::Div => binary_math(stack, |a,b| { (
                b != 0).then(|| a / b).ok_or(Error::DivisionByZero) 
            }),
            Builtin::Drop => { _ = stack.pop().ok_or(Error::StackUnderflow)?; Ok(()) },
            Builtin::Dup => { stack.push(stack.last().copied().ok_or(Error::StackUnderflow)?); Ok(()) },
            Builtin::Swap => (size >= 2).then(|| { 
                    stack.swap(size - 1, size - 2) 
                }).ok_or(Error::StackUnderflow),
            Builtin::Over => (size >= 2).then(|| { 
                    stack.push(stack[size - 2]) 
                }).ok_or(Error::StackUnderflow)
        }
    }
}

impl FromStr for Builtin {
    type Err = BuiltinError;

    /// Case Sensitive
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Builtin::Add),
            "-" => Ok(Builtin::Sub),
            "*" => Ok(Builtin::Mul),
            "/" => Ok(Builtin::Div),
            "drop" => Ok(Builtin::Drop),
            "dup" => Ok(Builtin::Dup),
            "swap" => Ok(Builtin::Swap),
            "over" => Ok(Builtin::Over),
            _ => Err(BuiltinError::InvalidBuiltin)
        }
    }
}

fn binary_math(stack: &mut Vec<i32>, op: fn(i32, i32) -> Result<i32, Error>) -> Result<(), Error> {
    if stack.len() < 2 {
        return Err(Error::StackUnderflow);
    } 
    let a = stack[0];
    let b = stack[1];
    stack.truncate(stack.len() - 2);
    stack.push(op(a, b)?);
    Ok(())
}


#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;
    use super::{Builtin, Error};

    #[test]
    fn stack_undeflow_on_empty_stack() {
        let mut stack = vec![];
        for builtin in Builtin::iter() {
            println!("{:?}", builtin);
            assert_eq!(builtin.call(&mut stack), Err(Error::StackUnderflow));
            assert_eq!(stack, vec![])
        }
    }
}