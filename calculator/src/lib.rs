use std::env;
use std::str::FromStr;

macro_rules! operation {
    ($a:expr,  $b:expr,   $s:tt ) => {
        match ($a, $b) {
            (Number::Float(a), Number::Float(b)) => Number::Float(a $s b),
            (Number::Int(a), Number::Float(b)) => Number::Float(a as f64 $s b),
            (Number::Float(a), Number::Int(b)) => Number::Float(a $s b as f64),
            (Number::Int(a), Number::Int(b)) => Number::Int(a $s b),
        }
    };
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Number {
    Int(i64),
    Float(f64),
}

impl FromStr for Number {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(".") {
            let n: f64 = s.parse()?;
            Ok(Self::Float(n))
        } else {
            let n: i64 = s.parse()?;
            Ok(Self::Int(n))
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    Plus,
    Minus,
    Times,
    DividedBy,
    Number(Number),
}

struct Tokenizer {
    input: Vec<char>,
    index: usize,
}

impl Tokenizer {
    fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            index: 0,
        }
    }

    fn next(&mut self) {
        self.index += 1;
    }

    fn get(&self) -> Option<char> {
        if self.index >= self.input.len() {
            None
        } else {
            Some(self.input[self.index])
        }
    }

    fn from_str(input: &str) -> Vec<Token> {
        let mut args = Self::new(input.to_string());
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(ch) = args.get() {
            if ch == '+' {
                tokens.push(Token::Plus);
            } else if ch == '-' {
                tokens.push(Token::Minus);
            } else if ch == '*' {
                tokens.push(Token::Times);
            } else if ch == '/' {
                tokens.push(Token::DividedBy);
            } else if ch.is_whitespace() {
                args.next();
                continue;
            } else {
                let mut number = String::new();
                while let Some(ch) = args.get() {
                    if "-+*/ ".contains(ch) {
                        break;
                    }

                    number.push(ch);
                    args.next();
                }

                let number = number.parse::<Number>().unwrap();
                tokens.push(Token::Number(number));
                continue;
            }

            args.next();
        }

        tokens
    }

    fn from_args() -> Vec<Token> {
        let mut args: Vec<_> = env::args().collect();
        args.remove(0);
        let args = args.join("");
        let tokens = Self::from_str(&args);
        tokens
    }
}

pub struct Calculator;

impl Calculator {
    pub fn from_args() -> Number {
        let tokens = Tokenizer::from_args();
        let mut tokens = tokens.iter();
        let mut numbers: Vec<Number> = Vec::new();

        loop {
            let token = tokens.next();

            if let None = token {
                break;
            }

            let token = token.unwrap();

            match token {
                Token::Number(n) => numbers.push(*n),
                Token::DividedBy => {
                    if let Token::Number(n) = tokens.next().unwrap() {
                        let sum = Self::div(numbers.pop().unwrap(), *n);
                        numbers.push(sum);
                    } else {
                        panic!("baf format")
                    }
                }
                Token::Minus => {
                    if let Token::Number(n) = tokens.next().unwrap() {
                        let sum = Self::rest(numbers.pop().unwrap(), *n);
                        numbers.push(sum);
                    } else {
                        panic!("baf format")
                    }
                }
                Token::Plus => {
                    if let Token::Number(n) = tokens.next().unwrap() {
                        let sum = Self::add(numbers.pop().unwrap(), *n);
                        numbers.push(sum);
                    } else {
                        panic!("baf format")
                    }
                }
                Token::Times => {
                    if let Token::Number(n) = tokens.next().unwrap() {
                        let sum = Self::mul(numbers.pop().unwrap(), *n);
                        numbers.push(sum);
                    } else {
                        panic!("baf format")
                    }
                }
            }
        }

        numbers[0]
    }

    fn mul(a: Number, b: Number) -> Number {
        //     match (a, b) {
        //         (Number::Float(a), Number::Float(b)) => Number::Float(a + b),
        //         (Number::Int(a), Number::Float(b)) => Number::Float(a as f64 + b),
        //         (Number::Float(a), Number::Int(b)) => Number::Float(a + b as f64),
        //         (Number::Int(a), Number::Int(b)) => Number::Int(a + b),
        //     }
        operation!(a, b, *)
    }
    fn add(a: Number, b: Number) -> Number {
        operation!(a, b, +)
    }

    fn div(a: Number, b: Number) -> Number {
        operation!(a, b, /)
    }
    fn rest(a: Number, b: Number) -> Number {
        operation!(a, b, -)
    }
}

#[cfg(test)]
mod test {
    use super::Token;
    use super::Tokenizer;
    use crate::Number;

    #[test]
    fn one_number() {
        assert_eq!(
            Vec::from([Token::Number(Number::Int(1))]),
            Tokenizer::from_str("1")
        );
    }

    #[test]
    fn sign_and_numbers() {
        assert_eq!(
            Vec::from([
                Token::Number(Number::Int(10)),
                Token::Plus,
                Token::Number(Number::Float(10.3)),
                Token::Times,
                Token::Number(Number::Int(4)),
                Token::DividedBy,
                Token::Number(Number::Int(2))
            ]),
            Tokenizer::from_str("10+10.3 * 4 / 2")
        );
    }

    #[test]
    fn operation_macro() {
        assert_eq!(
            operation!(Number::Int(1) ,Number::Int(2), *),
            Number::Int(2)
        );

        assert_eq!(
            operation!(Number::Int(2) ,Number::Int(2), *),
            Number::Int(4)
        );

        assert_eq!(
            operation!(Number::Float(2.0) ,Number::Int(2), +),
            Number::Float(4.0)
        );
    }
}
