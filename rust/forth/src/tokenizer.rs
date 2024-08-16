use std::str::FromStr;
use super::builtins::Builtin;

pub enum Token {
    Number(i32),
    Builtin(Builtin),
    Word(String)
}

pub enum TokenError { InvalidToken }

impl FromStr for Token {
    type Err = TokenError;

    fn from_str(token: &str) -> Result<Self, Self::Err> {
        let token = token.to_lowercase();
        if let Ok(number) = token.parse() {
            Ok(Token::Number(number))
        } else if let Ok(builtin) = token.parse() {
            Ok(Token::Builtin(builtin))
        } else if token != ":" {
            Ok(Token::Word(token))
        } else {
            Err(TokenError::InvalidToken)
        }
    }
}