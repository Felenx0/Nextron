use std::process::exit;
use crate::tokens::{Token, TokenType};

pub struct Lexer {
    index: usize,
    colune: usize,
    m_src: String
}

impl Lexer {
    pub fn new(src: String) -> Lexer {
        Lexer {
            index: 0,
            colune: 1,
            m_src: src
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut buf = String::new();
        let mut tokens: Vec<Token> = vec![];
        let mut is_str = false;

        while self.peek(0).is_some() {
            if !is_str {
                if self.peek(0).unwrap().is_alphabetic() {
                    buf.push(self.consume());
                    while self.peek(0).is_some() && self.peek(0).unwrap().is_alphanumeric() {
                        buf.push(self.consume());
                    }

                    if buf == "print" {
                        tokens.push(Token { _type: TokenType::Print, value: None, col: self.colune });
                    }
                    else if buf == "input" {
                        tokens.push(Token { _type: TokenType::Input, value: None, col: self.colune });
                    }
                    else if buf == "Number" {
                        tokens.push(Token { _type: TokenType::Number, value: None, col: self.colune });
                    }
                    else if buf == "Boolean" {
                        tokens.push(Token { _type: TokenType::Boolean, value: None, col: self.colune });
                    }
                    else if buf == "true" || buf == "false" {
                        tokens.push(Token { _type: TokenType::BooleanType, value: Some(buf.clone()), col: self.colune });
                    }
                    else if buf == "if" {
                        tokens.push(Token { _type: TokenType::If, value: None, col: self.colune });
                    }
                    else if buf == "else" {
                        tokens.push(Token { _type: TokenType::Else, value: None, col: self.colune });
                    }
                    else if buf == "&&" {
                        tokens.push(Token { _type: TokenType::And, value: None, col: self.colune });
                    }
                    else if buf == "||" {
                        tokens.push(Token { _type: TokenType::Or, value: None, col: self.colune });
                    }
                    else if buf == "var" {
                        tokens.push(Token { _type: TokenType::Var, value: None, col: self.colune });
                    }
                    else if buf == "while" {
                        tokens.push(Token { _type: TokenType::While, value: None, col: self.colune });
                    }
                    else if buf == "for" {
                        tokens.push(Token { _type: TokenType::For, value: None, col: self.colune });
                    }
                    else {
                        tokens.push(Token { _type: TokenType::Identifier, value: Some(buf.clone()), col: self.colune });
                    }

                    buf.clear();
                }
                else if self.peek(0).unwrap().is_numeric() {
                    let mut has_dot = false;
                    buf.push(self.consume());
                    while self.peek(0).is_some() && (self.peek(0).unwrap().is_numeric() || self.peek(0).unwrap() == '.') {
                        if self.peek(0).unwrap() == '.' && !has_dot {
                            has_dot = true;
                        }
                        else if self.peek(0).unwrap() == '.' && has_dot {
                            eprintln!("Invalid number value");
                            exit(101);
                        }
                        buf.push(self.consume());
                    }

                    tokens.push(Token { _type: TokenType::NumberType, value: Some(buf.clone()), col: self.colune });
                    buf.clear();
                }
                else if self.peek(0).unwrap() == '(' {
                    self.consume();
                    tokens.push(Token { _type: TokenType::OpenParam, value: None, col: self.colune });
                }
                else if self.peek(0).unwrap() == ')' {
                    self.consume();
                    tokens.push(Token { _type: TokenType::CloseParam, value: None, col: self.colune });
                }
                else if self.peek(0).unwrap() == '\'' || self.peek(0).unwrap() == '"' {
                    self.consume();
                    is_str = true;
                }
                else if self.peek(0).unwrap() == '+' {
                    self.consume();
                    if self.peek(0).unwrap() == '+' {
                        self.consume();
                        tokens.push(Token { _type: TokenType::PPlus, value: None, col: self.colune });
                    }
                    else {
                        tokens.push(Token { _type: TokenType::Add, value: None, col: self.colune });
                    }
                }
                else if self.peek(0).unwrap() == '-' {
                    self.consume();
                    if self.peek(0).unwrap().is_numeric() {
                        buf.push('-');
                        buf.push(self.consume());
                        while self.peek(0).is_some() && self.peek(0).unwrap().is_numeric() {
                            buf.push(self.consume());
                        }

                        tokens.push(Token { _type: TokenType::NumberType, value: Some(buf.clone()), col: self.colune });
                        buf.clear();
                    }
                    else if self.peek(0).unwrap() == '-' {
                        self.consume();
                        tokens.push(Token { _type: TokenType::MMinus, value: None, col: self.colune });
                    }
                    else {
                        tokens.push(Token { _type: TokenType::Minus, value: None, col: self.colune });
                    }
                }
                else if self.peek(0).unwrap() == '*' {
                    self.consume();
                    tokens.push(Token { _type: TokenType::Star, value: None, col: self.colune });
                }
                else if self.peek(0).unwrap() == '/' {
                    self.consume();
                    if self.peek(0).unwrap() == '/' {
                        self.consume();
                        while self.peek(0).is_some() && self.peek(0).unwrap() != '\r' && self.peek(1).is_some() && self.peek(1).unwrap() != '\n' {
                            if self.peek(0).unwrap() == '\r' {
                                break;
                            }
                            self.consume();
                        }
                    }
                    else {
                        tokens.push(Token { _type: TokenType::Div, value: None, col: self.colune });
                    }
                }
                else if self.peek(0).unwrap() == '=' {
                    self.consume();
                    if self.peek(0).unwrap() == '=' {
                        self.consume();
                        tokens.push(Token { _type: TokenType::EqEq, value: None, col: self.colune });
                    }
                    else {
                        tokens.push(Token { _type: TokenType::Equals, value: None, col: self.colune });
                    }
                }
                else if self.peek(0).unwrap() == '!' {
                    self.consume();
                    if self.peek(0).unwrap() == '=' {
                        self.consume();
                        tokens.push(Token { _type: TokenType::Diff, value: None, col: self.colune });
                    }
                }
                else if self.peek(0).unwrap() == '.' {
                    self.consume();
                    if self.peek(0).unwrap().is_numeric() {
                        buf.push_str("0.");
                        buf.push(self.consume());
                        while self.peek(0).is_some() && self.peek(0).unwrap().is_numeric() {
                            buf.push(self.consume());
                        }

                        tokens.push(Token { _type: TokenType::NumberType, value: Some(buf.clone()), col: self.colune });
                        buf.clear();
                    }
                    else {
                        println!("PONTO");
                        tokens.push(Token { _type: TokenType::Dot, value: None, col: self.colune });
                    }
                }
                else if self.peek(0).unwrap() == '{' {
                    self.consume();
                    tokens.push(Token { _type: TokenType::OpenCBrackets, value: None, col: self.colune });
                }
                else if self.peek(0).unwrap() == '}' {
                    self.consume();
                    tokens.push(Token { _type: TokenType::CloseCBrackets, value: None, col: self.colune });
                }
                else if self.peek(0).unwrap() == '<' {
                    self.consume();
                    if self.peek(0).unwrap() == '=' {
                        self.consume();
                        tokens.push(Token { _type: TokenType::LessEq, value: None, col: self.colune });
                    }
                    else {
                        tokens.push(Token { _type: TokenType::Less, value: None, col: self.colune });
                    }
                }
                else if self.peek(0).unwrap() == '>' {
                    self.consume();
                    if self.peek(0).unwrap() == '=' {
                        self.consume();
                        tokens.push(Token { _type: TokenType::BiggerEq, value: None, col: self.colune });
                    }
                    else {
                        tokens.push(Token { _type: TokenType::Bigger, value: None, col: self.colune });
                    }
                }
                else if self.peek(0).unwrap() == '&' && self.peek(1).is_some() && self.peek(1).unwrap() == '&' {
                    self.consume();
                    self.consume();
                    tokens.push(Token { _type: TokenType::And, value: None, col: self.colune });
                }
                else if self.peek(0).unwrap() == '|' && self.peek(1).is_some() && self.peek(1).unwrap() == '|' {
                    self.consume();
                    self.consume();
                    tokens.push(Token { _type: TokenType::Or, value: None, col: self.colune });
                }
                else if self.peek(0).unwrap() == ';' {
                    self.consume();
                    tokens.push(Token { _type: TokenType::Semi, value: None, col: self.colune });
                }
                else if self.peek(0).unwrap().is_whitespace() {
                    self.consume();
                }
                if self.peek(0).is_some() && self.peek(0).unwrap() == '\r' && self.peek(1).is_some() && self.peek(1).unwrap() == '\n' {
                    self.consume();
                    self.colune = 0;
                    tokens.push(Token { _type: TokenType::NwLine, value: None, col: self.colune+1 });
                }
            }
            else {
                while self.peek(0).is_some() && (self.peek(0).unwrap() != '\'' && self.peek(0).unwrap() != '"') {
                    if self.peek(0).unwrap() == '\'' || self.peek(0).unwrap() == '"' {
                        //println!("Oi");
                        break;
                    }

                    buf.push(self.consume());
                    //println!("{buf}");
                }

                if self.peek(0).is_some() && (self.peek(0).unwrap() == '\'' || self.peek(0).unwrap() == '"') {
                    self.consume();
                    is_str = false;
                    tokens.push(Token { _type: TokenType::StringType, value: Some(buf.clone()), col: self.colune });
                    buf.clear();
                    //println!("Fim de Str, {is_str}");
                }
            }
        }

        println!("{:?}", tokens);
        tokens
    }

    fn peek(&mut self, offset: usize) -> Option<char> {
        self.m_src.chars().nth(self.index + offset)
    }

    fn consume(&mut self) -> char {
        self.index += 1;
        self.colune += 1;
        //println!("COL: {}", self.colune);
        self.m_src.chars().nth(self.index-1).unwrap()
    }
}
