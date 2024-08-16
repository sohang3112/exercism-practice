pub mod builtins;
mod tokenizer;

use std::collections::HashMap;
use crate::tokenizer::{Token, TokenError};
use crate::builtins::{Builtin, Error};

pub type Value = i32;

pub struct Forth {
    stack: Vec<i32>,
    words: HashMap<String, Vec<String>>
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            words: HashMap::new()
        }
    }

    pub fn stack(&self) -> &[i32] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result<(), Error> {
        for token_str in input.split_whitespace() {
            match token_str.parse() {
                Ok(Token::Number(number)) => self.stack.push(number),
                Ok(Token::Builtin(builtin)) => builtin.call(&mut self.stack)?,
                Ok(Token::Word(word)) => unimplemented!("Calling Word"),
                Err(TokenError::InvalidToken) => unimplemented!("Defining new word")
            }
        }
        Ok(())
    }
}

fn define_word<'a, I>(tokens: &mut I, words: &mut Vec<(String, Vec<Token>)>) -> Option<()>
where 
    I: Iterator<Item = &'a str> 
{                
    let word = tokens.next()?.to_lowercase();
    let definition: Option<Vec<Builtin>> = tokens.take_while(|t| *t != ";")
                                          .map(|token| { token.parse().ok() })
                                          .collect();
    tokens.next()?;          // skip ";" -> definiton end
    Some(())
}