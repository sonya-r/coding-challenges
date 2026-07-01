use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Number {
    Float(f32),
    Integer(i32),
}

#[derive(Debug, PartialEq)]
pub enum JsonType {
    String(String),
    Number(Number),
    Boolean(bool),
    Object(HashMap<String, Box<JsonType>>),
    Array(Vec<Box<JsonType>>),
    Null,
}

#[derive(Debug, PartialEq)]
enum Token {
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Colon,
    Comma,
    String(String),
    Number(Number),
    Boolean(bool),
    Null,
}

#[derive(Debug)]
pub enum JsonError {
    Empty,
    BadFormat,
}

pub struct Json {
    tokens: Vec<Token>,
    i: usize,
}

impl Json {
    pub fn parse(input: &str) -> Result<JsonType, JsonError> {
        let mut json = Self {
            i: 0,
            tokens: json_lexer(input)?,
        };

        json.parse_token()
    }

    fn next(&mut self) -> &Token {
        let x = self.i;
        self.i += 1;
        return &self.tokens[x];
    }
    fn back(&mut self) {
        self.i -= 1;
    }

    fn parse_token(&mut self) -> Result<JsonType, JsonError> {
        let token = self.next();

        match token {
            Token::Boolean(v) => Ok(JsonType::Boolean(*v)),
            Token::Number(v) => Ok(JsonType::Number(*v)),
            Token::String(s) => Ok(JsonType::String(s.clone())),
            Token::Null => Ok(JsonType::Null),
            Token::OpenBrace => self.parse_object(),
            Token::OpenBracket => self.parse_array(),
            _ => Err(JsonError::BadFormat),
        }
    }

    fn parse_object(&mut self) -> Result<JsonType, JsonError> {
        let mut object: HashMap<String, Box<JsonType>> = HashMap::new();

        loop {
            let next = self.next();

            if *next == Token::CloseBrace {
                break;
            } else {
                self.back();
            }

            if let JsonType::String(key) = self.parse_token()? {
                if let Token::Colon = self.next() {
                    let value = self.parse_token()?;
                    object.insert(key, Box::from(value));
                } else {
                    self.bad_format()?;
                }
            } else {
                self.bad_format()?;
            }

            let next = self.next();

            if *next == Token::Comma {
                continue;
            }

            if *next == Token::CloseBrace {
                break;
            } else {
                self.bad_format()?;
            }
        }

        Ok(JsonType::Object(object))
    }

    fn parse_array(&mut self) -> Result<JsonType, JsonError> {
        let mut array: Vec<Box<JsonType>> = Vec::new();

        loop {
            let next = self.next();

            if *next == Token::CloseBracket {
                break;
            } else {
                self.back();
            }

            let val = self.parse_token()?;
            array.push(Box::from(val));

            let next = self.next();

            if *next == Token::Comma {
                continue;
            }

            if *next == Token::CloseBracket {
                break;
            } else {
                self.bad_format()?;
            }
        }

        Ok(JsonType::Array(array))
    }

    fn bad_format(&self) -> Result<(), JsonError> {
        return Err(JsonError::BadFormat);
    }
}
fn json_lexer(input: &str) -> Result<Vec<Token>, JsonError> {
    let input: Vec<char> = input.trim().chars().collect();
    let mut tokens: Vec<Token> = Vec::new();
    let mut i = 0;

    if input.is_empty() {
        return Err(JsonError::Empty);
    }

    while i < input.len() {
        let chr = input[i];

        if chr.is_whitespace() {
            i += 1;
            continue;
        }

        if chr == '{' {
            tokens.push(Token::OpenBrace);
        } else if chr == '}' {
            tokens.push(Token::CloseBrace);
        } else if chr == '[' {
            tokens.push(Token::OpenBracket);
        } else if chr == ']' {
            tokens.push(Token::CloseBracket);
        } else if chr == ':' {
            tokens.push(Token::Colon);
        } else if chr == ',' {
            tokens.push(Token::Comma);
        } else if chr == '"' {
            let mut string = String::new();
            i += 1;
            while i < input.len() {
                let chr = input[i];
                if chr == '"' {
                    break;
                }
                string.push(chr);
                i += 1;
            }
            tokens.push(Token::String(string));
        } else if chr.is_digit(10) || chr == '-' {
            let mut string = String::from(chr);
            i += 1;

            while i < input.len() {
                let chr = input[i];

                if !chr.is_digit(10) && chr != '.' {
                    i -= 1;
                    break;
                }

                if string.contains('.') && chr == '.' {}

                string.push(chr);
                i += 1;
            }
            if string.contains('.') {
                tokens.push(Token::Number(Number::Float(string.parse::<f32>().unwrap())));
            } else {
                tokens.push(Token::Number(Number::Integer(
                    string.parse::<i32>().unwrap(),
                )));
            }
        } else if chr == 'f' {
            let len = "false".len();
            let string: String = input[i..i + len].iter().collect();
            if string == "false" {
                tokens.push(Token::Boolean(false));
                i += len - 1;
            } else {
                return Err(JsonError::BadFormat);
            }
        } else if chr == 't' {
            let len = "true".len();
            let string: String = input[i..i + len].iter().collect();
            if string == "true" {
                tokens.push(Token::Boolean(true));
                i += len - 1;
            } else {
                return Err(JsonError::BadFormat);
            }
        } else if chr == 'n' {
            let len = "null".len();
            let string: String = input[i..i + len].iter().collect();
            if string == "null" {
                tokens.push(Token::Null);
                i += len - 1;
            } else {
                return Err(JsonError::BadFormat);
            }
        } else {
            return Err(JsonError::BadFormat);
        }

        i += 1;
    }

    // for token in &tokens {
    //     println!("{token:?}")
    // }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::Json;
    use std::{fs::File, io::Read};

    fn read_file(filename: &str) -> String {
        let mut file = File::open(filename).unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();
        input
    }

    #[test]
    fn valid_json() {
        let input = read_file("test.json");
        let json = Json::parse(&input);

        assert!(json.is_ok());

        println!("{json:?}");
    }
}
