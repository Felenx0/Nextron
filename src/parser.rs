use crate::tokens::{Token, TokenType};
use eval::eval;
use std::collections::HashMap;
use std::process::exit;
use std::io;
use std::io::Write;

#[derive(Debug)]
struct Vars {
    _type: TokenType,
    value: String,
}

pub struct Parser {
    index: usize,
    c_col: usize,
    line: usize,
    m_tokens: Vec<Token>,
    m_vars: HashMap<String, Vars>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            index: 0,
            c_col: 1,
            line: 1,
            m_tokens: tokens,
            m_vars: HashMap::new(),
        }
    }

    fn parse_bin_expr(&mut self) -> String {
        let mut expr = String::new();

        if self.peek(0).unwrap()._type == TokenType::NumberType {
            expr.push_str(self.consume().value.clone().unwrap().as_str());
        }
        else if self.peek(0).unwrap()._type == TokenType::Identifier {
            let ident = self.consume().value.clone().unwrap();

            if !self.m_vars.contains_key(ident.clone().as_str()) {
                self.print_error(format!("Identifier not exist: {ident}").as_str());
            }

            if let Some(var) = self.m_vars.get(ident.as_str()) {
                if var._type == TokenType::NumberType {
                    expr.push_str(var.value.clone().as_str());
                }
            }
        }

        if self.peek(0).unwrap()._type == TokenType::Add {
            self.consume();
            expr.push('+');
        }
        else if self.peek(0).unwrap()._type == TokenType::Minus {
            self.consume();
            expr.push('-');
        }
        else if self.peek(0).unwrap()._type == TokenType::Star {
            self.consume();
            expr.push('*');
        }
        else if self.peek(0).unwrap()._type == TokenType::Div {
            self.consume();
            expr.push('/');
        }

        if self.peek(0).unwrap()._type == TokenType::NumberType {
            expr.push_str(self.consume().value.clone().unwrap().as_str());
        }
        else if self.peek(0).unwrap()._type == TokenType::Identifier {
            let ident = self.consume().value.clone().unwrap();

            if !self.m_vars.contains_key(ident.clone().as_str()) {
                self.print_error(format!("Identifier not exist: {ident}").as_str());
            }

            if let Some(var) = self.m_vars.get(ident.as_str()) {
                if var._type == TokenType::NumberType {
                    expr.push_str(var.value.clone().as_str());
                }
            }
        }

        while self.peek(0).is_some() && self.peek(0).unwrap()._type == TokenType::Add || self.peek(0).unwrap()._type == TokenType::Minus || self.peek(0).unwrap()._type == TokenType::NumberType || self.peek(0).unwrap()._type == TokenType::Star || self.peek(0).unwrap()._type == TokenType::Div {
            if self.peek(0).unwrap()._type == TokenType::Add {
                self.consume();
                expr.push('+');
            }
            else if self.peek(0).unwrap()._type == TokenType::Minus {
                self.consume();
                expr.push('-');
            }
            else if self.peek(0).unwrap()._type == TokenType::Star {
                self.consume();
                expr.push('*');
            }
            else if self.peek(0).unwrap()._type == TokenType::Div {
                self.consume();
                expr.push('/');
            }

            if self.peek(0).unwrap()._type == TokenType::NumberType {
                expr.push_str(self.consume().value.clone().unwrap().as_str());
            }
            else if self.peek(0).unwrap()._type == TokenType::Identifier {
                let ident = self.consume().value.clone().unwrap();

                if !self.m_vars.contains_key(ident.clone().as_str()) {
                    self.print_error(format!("Identifier not exist: {ident}").as_str());
                }

                if let Some(var) = self.m_vars.get(ident.as_str()) {
                    if var._type == TokenType::NumberType {
                        expr.push_str(var.value.clone().as_str());
                    }
                }
            }
        }

        //println!("{expr}");

        let res = eval(expr.as_str()).unwrap();

        //println!("{:?}", res);
        res.to_string()
    }

    fn parse_vars(&mut self) {
        if self.peek(0).unwrap()._type == TokenType::Var {
            self.consume();
            if self.peek(0).is_some() && self.peek(0).unwrap()._type == TokenType::Identifier {
                let ident = self.consume().value.clone().unwrap();
                let mut v = String::new();

                if self.m_vars.contains_key(ident.clone().as_str()) {
                    self.print_error(format!("Identifier already used: {ident}").as_str());
                }

                if self.peek(0).is_some() && self.peek(0).unwrap()._type == TokenType::Equals {
                    self.consume();

                    if self.peek(0).unwrap()._type == TokenType::StringType {
                        v = self.consume().value.clone().unwrap();

                        self.m_vars.insert(ident, Vars { _type: TokenType::StringType, value: v, });
                    } else if self.peek(0).unwrap()._type == TokenType::NumberType {
                        if self.peek(1).is_some() && (self.peek(1).unwrap()._type == TokenType::Add || self.peek(1).unwrap()._type == TokenType::Minus || self.peek(1).unwrap()._type == TokenType::Star || self.peek(1).unwrap()._type == TokenType::Div) {
                            v = self.parse_bin_expr();

                            self.m_vars.insert(
                                ident,
                                Vars {
                                    _type: TokenType::NumberType,
                                    value: v,
                                },
                            );
                        } else {
                            if self.peek(0).unwrap().value.clone().unwrap().contains('.') {
                                let f = self
                                    .consume()
                                    .value
                                    .clone()
                                    .unwrap()
                                    .parse::<f64>()
                                    .unwrap();
                                v = f.to_string();
                            } else {
                                v = self.consume().value.clone().unwrap();
                            }

                            self.m_vars.insert(
                                ident,
                                Vars {
                                    _type: TokenType::NumberType,
                                    value: v,
                                },
                            );
                        }
                    } else if self.peek(0).unwrap()._type == TokenType::BooleanType {
                        v = self.consume().value.clone().unwrap();

                        self.m_vars.insert(ident, Vars { _type: TokenType::BooleanType, value: v });
                    } else if self.peek(0).unwrap()._type == TokenType::Identifier {
                        if self.peek(1).is_some() && (self.peek(1).unwrap()._type == TokenType::Add || self.peek(1).unwrap()._type == TokenType::Minus || self.peek(1).unwrap()._type == TokenType::Star || self.peek(1).unwrap()._type == TokenType::Div)
                        {
                            v = self.parse_bin_expr();
                            self.m_vars.insert(
                                ident,
                                Vars {
                                    _type: TokenType::NumberType,
                                    value: v,
                                },
                            );
                        } else {
                            let m_ident = self.consume().value.clone().unwrap();

                            if !self.m_vars.contains_key(m_ident.clone().as_str()) {
                                self.print_error(format!("Identifier not exist: {m_ident}").as_str());
                            }

                            if let Some(var) = self.m_vars.get(m_ident.as_str()) {
                                if var._type == TokenType::StringType {
                                    v = var.value.clone();

                                    self.m_vars.insert(
                                        ident,
                                        Vars {
                                            _type: TokenType::StringType,
                                            value: v,
                                        },
                                    );
                                } else if var._type == TokenType::NumberType {
                                    v = var.value.clone();

                                    self.m_vars.insert(
                                        ident,
                                        Vars {
                                            _type: TokenType::NumberType,
                                            value: v,
                                        },
                                    );
                                }
                            }
                        }
                    } else if self.peek(0).unwrap()._type == TokenType::Input {
                        let mut inp_text = String::new();
                        self.consume();

                        self.try_consume(TokenType::OpenParam, "Expected `(`");

                        if self.peek(0).is_some() && self.peek(0).unwrap()._type == TokenType::StringType {
                            inp_text = self.consume().value.clone().unwrap();
                        }

                        self.try_consume(TokenType::CloseParam, "Expected `)`");

                        print!("{inp_text}");
                        io::stdout().flush().unwrap();
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        input.pop();
                        input.pop();

                        self.m_vars.insert(ident, Vars { _type: TokenType::StringType, value: input });
                    } else if self.peek(0).unwrap()._type == TokenType::Number {
                        self.consume();

                        self.try_consume(TokenType::OpenParam, "Expected `(`");

                        if self.peek(0).unwrap()._type == TokenType::Input {
                            let mut inp_text = String::new();
                            self.consume();

                            self.try_consume(TokenType::OpenParam, "Expected `(`");

                            if self.peek(0).is_some() && self.peek(0).unwrap()._type == TokenType::StringType {
                                inp_text = self.consume().value.clone().unwrap();
                            }

                            self.try_consume(TokenType::CloseParam, "Expected `)`");

                            print!("{inp_text}");
                            io::stdout().flush().unwrap();
                            let mut input = String::new();
                            io::stdin().read_line(&mut input).expect("Failed to read line");
                            input.pop();
                            input.pop();

                            if input.chars().all(|c| c.is_numeric() || c == '.') {
                                if input.contains('.') {
                                    let v = input.parse::<f64>().unwrap();

                                    input = v.to_string();
                                }
                            } else {
                                self.print_error("Invalid value to convert to number");
                            }


                            self.m_vars.insert(ident, Vars { _type: TokenType::NumberType, value: input });
                        } else if self.peek(0).unwrap()._type == TokenType::StringType {
                            let mut value = self.consume().value.clone().unwrap();

                            if value.chars().all(|c| c.is_numeric() || c == '.') {
                                if value.contains('.') {
                                    let v = value.parse::<f64>().unwrap();

                                    value = v.to_string();
                                }
                            } else {
                                self.print_error("Invalid value to convert to number");
                            }

                            self.m_vars.insert(ident, Vars { _type: TokenType::NumberType, value });
                        }

                        self.try_consume(TokenType::CloseParam, "Expected `)`");
                    } else if self.peek(0).unwrap()._type == TokenType::Boolean {
                        self.consume();

                        self.try_consume(TokenType::OpenParam, "Expected `(`");

                        if self.peek(0).unwrap()._type == TokenType::Input {
                            let mut inp_text = String::new();
                            self.consume();

                            self.try_consume(TokenType::OpenParam, "Expected `(`");

                            if self.peek(0).is_some() && self.peek(0).unwrap()._type == TokenType::StringType {
                                inp_text = self.consume().value.clone().unwrap();
                            }

                            self.try_consume(TokenType::CloseParam, "Expected `)`");

                            print!("{inp_text}");
                            io::stdout().flush().unwrap();
                            let mut input = String::new();
                            io::stdin().read_line(&mut input).expect("Failed to read line");
                            input.pop();
                            input.pop();

                            if input != "true" && input != "false" {
                                self.print_error("Invalid value to convert to boolean");
                            }

                            self.m_vars.insert(ident, Vars { _type: TokenType::BooleanType, value: input });
                        } else if self.peek(0).unwrap()._type == TokenType::StringType {
                            let mut value = self.consume().value.clone().unwrap();

                            if value != "true" && value != "false" {
                                self.print_error("Invalid value to convert to boolean");
                            }

                            self.m_vars.insert(ident, Vars { _type: TokenType::BooleanType, value });
                        }

                        self.try_consume(TokenType::CloseParam, "Expected `)`");
                    }
                }

                println!("{:?}", self.m_vars);

                self.try_consume(TokenType::Semi, "Expected `;`");
            }
        }
        else {
            let ident = self.consume().value.clone().unwrap();
            let mut v = String::new();

            if !self.m_vars.contains_key(ident.clone().as_str()) {
                self.print_error(format!("Identifier not exist: {ident}").as_str());
            }

            if self.peek(0).is_some() && self.peek(0).unwrap()._type == TokenType::Equals {
                self.consume();

                if self.peek(0).unwrap()._type == TokenType::StringType {
                    v = self.consume().value.clone().unwrap();

                    self.m_vars.insert(ident, Vars { _type: TokenType::StringType, value: v, });
                }
                else if self.peek(0).unwrap()._type == TokenType::NumberType {
                    if self.peek(1).is_some() && (self.peek(1).unwrap()._type == TokenType::Add || self.peek(1).unwrap()._type == TokenType::Minus || self.peek(1).unwrap()._type == TokenType::Star || self.peek(1).unwrap()._type == TokenType::Div){
                        v = self.parse_bin_expr();

                        self.m_vars.insert(
                            ident,
                            Vars {
                                _type: TokenType::NumberType,
                                value: v,
                            },
                        );
                    } else {
                        if self.peek(0).unwrap().value.clone().unwrap().contains('.') {
                            let f = self
                                .consume()
                                .value
                                .clone()
                                .unwrap()
                                .parse::<f64>()
                                .unwrap();
                            v = f.to_string();
                        } else {
                            v = self.consume().value.clone().unwrap();
                        }

                        self.m_vars.insert(
                            ident,
                            Vars {
                                _type: TokenType::NumberType,
                                value: v,
                            },
                        );
                    }
                }
                else if self.peek(0).unwrap()._type == TokenType::BooleanType {
                    v = self.consume().value.clone().unwrap();

                    self.m_vars.insert(ident, Vars { _type: TokenType::BooleanType, value: v });
                }
                else if self.peek(0).unwrap()._type == TokenType::Identifier {
                    if self.peek(1).is_some() && (self.peek(1).unwrap()._type == TokenType::Add || self.peek(1).unwrap()._type == TokenType::Minus || self.peek(1).unwrap()._type == TokenType::Star || self.peek(1).unwrap()._type == TokenType::Div)
                    {
                        v = self.parse_bin_expr();
                        self.m_vars.insert(
                            ident,
                            Vars {
                                _type: TokenType::NumberType,
                                value: v,
                            },
                        );
                    } else {
                        let m_ident = self.consume().value.clone().unwrap();

                        if !self.m_vars.contains_key(m_ident.clone().as_str()) {
                            self.print_error(format!("Identifier not exist: {m_ident}").as_str());
                        }

                        if let Some(var) = self.m_vars.get(m_ident.as_str()) {
                            if var._type == TokenType::StringType {
                                v = var.value.clone();

                                self.m_vars.insert(
                                    ident,
                                    Vars {
                                        _type: TokenType::StringType,
                                        value: v,
                                    },
                                );
                            } else if var._type == TokenType::NumberType {
                                v = var.value.clone();

                                self.m_vars.insert(
                                    ident,
                                    Vars {
                                        _type: TokenType::NumberType,
                                        value: v,
                                    },
                                );
                            }
                        }
                    }
                }
                else if self.peek(0).unwrap()._type == TokenType::Input {
                    let mut inp_text = String::new();
                    self.consume();

                    self.try_consume(TokenType::OpenParam, "Expected `(`");

                    if self.peek(0).is_some() && self.peek(0).unwrap()._type == TokenType::StringType {
                        inp_text = self.consume().value.clone().unwrap();
                    }

                    self.try_consume(TokenType::CloseParam, "Expected `)`");

                    print!("{inp_text}");
                    io::stdout().flush().unwrap();
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    input.pop();
                    input.pop();

                    self.m_vars.insert(ident, Vars { _type: TokenType::StringType, value: input });
                }
                else if self.peek(0).unwrap()._type == TokenType::Number {
                    self.consume();

                    self.try_consume(TokenType::OpenParam, "Expected `(`");

                    if self.peek(0).unwrap()._type == TokenType::Input {
                        let mut inp_text = String::new();
                        self.consume();

                        self.try_consume(TokenType::OpenParam, "Expected `(`");

                        if self.peek(0).is_some() && self.peek(0).unwrap()._type == TokenType::StringType {
                            inp_text = self.consume().value.clone().unwrap();
                        }

                        self.try_consume(TokenType::CloseParam, "Expected `)`");

                        print!("{inp_text}");
                        io::stdout().flush().unwrap();
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        input.pop();
                        input.pop();

                        if input.chars().all(|c| c.is_numeric() || c == '.') {
                            if input.contains('.') {
                                let v = input.parse::<f64>().unwrap();

                                input = v.to_string();
                            }
                        }
                        else {
                            self.print_error("Invalid value to convert to number");
                        }


                        self.m_vars.insert(ident, Vars { _type: TokenType::NumberType, value: input });
                    }
                    else if self.peek(0).unwrap()._type == TokenType::StringType {
                        let mut value = self.consume().value.clone().unwrap();

                        if value.chars().all(|c| c.is_numeric() || c == '.') {
                            if value.contains('.') {
                                let v = value.parse::<f64>().unwrap();

                                value = v.to_string();
                            }
                        }
                        else {
                            self.print_error("Invalid value to convert to number");
                        }

                        self.m_vars.insert(ident, Vars { _type: TokenType::NumberType, value });
                    }

                    self.try_consume(TokenType::CloseParam, "Expected `)`");
                }
                else if self.peek(0).unwrap()._type == TokenType::Boolean {
                    self.consume();

                    self.try_consume(TokenType::OpenParam, "Expected `(`");

                    if self.peek(0).unwrap()._type == TokenType::Input {
                        let mut inp_text = String::new();
                        self.consume();

                        self.try_consume(TokenType::OpenParam, "Expected `(`");

                        if self.peek(0).is_some() && self.peek(0).unwrap()._type == TokenType::StringType {
                            inp_text = self.consume().value.clone().unwrap();
                        }

                        self.try_consume(TokenType::CloseParam, "Expected `)`");

                        print!("{inp_text}");
                        io::stdout().flush().unwrap();
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        input.pop();
                        input.pop();

                        if input != "true" && input != "false" {
                            self.print_error("Invalid value to convert to boolean");
                        }

                        self.m_vars.insert(ident, Vars { _type: TokenType::BooleanType, value: input });
                    }
                    else if self.peek(0).unwrap()._type == TokenType::StringType {
                        let mut value = self.consume().value.clone().unwrap();

                        if value != "true" && value != "false" {
                            self.print_error("Invalid value to convert to boolean");
                        }

                        self.m_vars.insert(ident, Vars { _type: TokenType::BooleanType, value });
                    }

                    self.try_consume(TokenType::CloseParam, "Expected `)`");
                }
            }
            else if self.peek(0).is_some() && self.peek(0).unwrap()._type == TokenType::PPlus {
                self.consume();
                if let Some(var) = self.m_vars.get(ident.as_str()) {
                    if var._type == TokenType::NumberType {
                        let mut expr = var.value.clone();
                        expr.push_str(" + 1");

                        let v = eval(expr.as_str()).unwrap();

                        self.m_vars.insert(ident, Vars { _type: TokenType::NumberType, value: v.to_string() });
                    }
                    else {
                        self.print_error("Invalid variable type");
                    }
                }
            }
            else if self.peek(0).is_some() && self.peek(0).unwrap()._type == TokenType::MMinus {
                self.consume();
                if let Some(var) = self.m_vars.get(ident.as_str()) {
                    if var._type == TokenType::NumberType {
                        let mut expr = var.value.clone();
                        expr.push_str(" - 1");

                        let v = eval(expr.as_str()).unwrap();

                        self.m_vars.insert(ident, Vars { _type: TokenType::NumberType, value: v.to_string() });
                    }
                    else {
                        self.print_error("Invalid variable type");
                    }
                }
            }

            println!("{:?}", self.m_vars);

            self.try_consume(TokenType::Semi, "Expected `;`");
        }
    }

    fn parse_print(&mut self) -> String {
        let mut contents = String::new();

        self.try_consume(TokenType::OpenParam, "Expected `(`");

        if self.peek(0).unwrap()._type == TokenType::StringType {
            contents.push_str(self.consume().value.clone().unwrap().as_str());
        }
        else if self.peek(0).unwrap()._type == TokenType::NumberType {
            if self.peek(1).is_some() && self.peek(2).is_some() && (self.peek(1).unwrap()._type == TokenType::Add || self.peek(1).unwrap()._type == TokenType::Minus || self.peek(1).unwrap()._type == TokenType::Star || self.peek(1).unwrap()._type == TokenType::Div) && self.peek(2).unwrap()._type == TokenType::NumberType {
                contents.push_str(self.parse_bin_expr().as_str());
            }
            else if self.peek(1).unwrap()._type == TokenType::NumberType && self.peek(1).unwrap().value.clone().unwrap().contains('-') {
                contents.push_str(self.parse_bin_expr().as_str());
            }
            else {
                if self.peek(0).unwrap().value.clone().unwrap().contains('.') {
                    let v = self.consume().value.clone().unwrap().parse::<f64>().unwrap();
                    contents.push_str(v.to_string().as_str());
                } else {
                    contents.push_str(self.consume().value.clone().unwrap().as_str());
                }
            }
        }
        else if self.peek(0).unwrap()._type == TokenType::Identifier {
            let ident = self.consume().value.clone().unwrap();

            if !self.m_vars.contains_key(ident.clone().as_str()) {
                self.print_error(format!("Identifier not exist: {ident}").as_str());
            }

            contents.push_str(self.m_vars.get(ident.as_str()).unwrap().value.as_str());
        }

        while self.peek(0).is_some() && self.peek(0).unwrap()._type != TokenType::CloseParam {
            if self.peek(0).unwrap()._type == TokenType::Add {
                self.consume();
                if self.peek(0).unwrap()._type == TokenType::StringType {
                    contents.push_str(self.consume().value.clone().unwrap().as_str());
                }
                else if self.peek(0).unwrap()._type == TokenType::NumberType {
                    if self.peek(1).is_some() && self.peek(2).is_some() && (self.peek(1).unwrap()._type == TokenType::Add || self.peek(1).unwrap()._type == TokenType::Minus || self.peek(1).unwrap()._type == TokenType::Star || self.peek(1).unwrap()._type == TokenType::Div) && self.peek(2).unwrap()._type == TokenType::NumberType {
                        contents.push_str(self.parse_bin_expr().as_str());
                    }
                    else if self.peek(1).unwrap()._type == TokenType::NumberType {
                        contents.push_str(self.parse_bin_expr().as_str());
                    }
                    else {
                        contents.push_str(self.consume().value.clone().unwrap().as_str());
                    }
                }
                else if self.peek(0).unwrap()._type == TokenType::Identifier {
                    let ident = self.consume().value.clone().unwrap();

                    if !self.m_vars.contains_key(ident.clone().as_str()) {
                        self.print_error(format!("Identifier not exist: {ident}").as_str());
                    }

                    contents.push_str(self.m_vars.get(ident.as_str()).unwrap().value.as_str());
                }
            }
            else {
                self.print_error("Expected `+` to concatenate");
            }
        }

        self.try_consume(TokenType::CloseParam, "Expected `)`");
        self.try_consume(TokenType::Semi, "Expected `;`");

        contents
    }

    fn parse_parse(&mut self) {
        //println!("{:?}", self.peek(0));
        if self.peek(0).unwrap()._type == TokenType::Print {
            self.consume();
            let contents = self.parse_print();
            println!("{contents}");
        }
        else if self.peek(0).unwrap()._type == TokenType::Identifier {
            self.parse_vars();
        }
        else if self.peek(0).unwrap()._type == TokenType::If {
            self.parse_if();
        }
        else if self.peek(0).unwrap()._type == TokenType::NwLine {
            self.consume();
            //println!("{}", self.line);
            self.line += 1;
        }
    }

    fn parse_parse_w(&mut self, e_o_w: bool) {
        //println!("{:?}", self.peek(0));
        if self.peek(0).unwrap()._type == TokenType::Print {
            self.consume();
            let contents = self.parse_print();
            println!("{contents}");
        }
        else if self.peek(0).unwrap()._type == TokenType::Identifier {
            self.parse_vars();
        }
        else if self.peek(0).unwrap()._type == TokenType::If {
            self.parse_if();
        }
        else if self.peek(0).unwrap()._type == TokenType::NwLine {
            self.consume();
            //println!("L: {}", self.line);
            if !e_o_w {
                self.line += 1;
            }
        }
    }

    fn parse_if(&mut self) {
        let mut res = true;

        if self.peek(0).unwrap()._type == TokenType::If {
            let mut cond = String::new();
            self.consume();

            self.try_consume(TokenType::OpenParam, "Expected `(`");

            if self.peek(0).unwrap()._type == TokenType::NumberType {
                cond.push_str(self.consume().value.clone().unwrap().as_str());
            }
            else if self.peek(0).unwrap()._type == TokenType::BooleanType {
                cond.push_str(self.consume().value.clone().unwrap().as_str());
            }
            else if self.peek(0).unwrap()._type == TokenType::StringType {
                cond.push_str(self.consume().value.clone().unwrap().as_str());
            }
            else if self.peek(0).unwrap()._type == TokenType::Identifier {
                let ident = self.consume().value.clone().unwrap();

                if !self.m_vars.contains_key(ident.clone().as_str()) {
                    self.print_error(format!("Identifier not exist: {ident}").as_str());
                }

                if let Some(var) = self.m_vars.get(ident.as_str()) {
                    if var._type == TokenType::NumberType {
                        cond.push_str(var.value.clone().as_str());
                    }
                    else if var._type == TokenType::BooleanType {
                        cond.push_str(var.value.clone().as_str());
                    }
                    else if var._type == TokenType::StringType {
                        cond.push_str(var.value.clone().as_str());
                    }
                }
            }
            else {
                self.print_error("Expected a condition");
            }

            if self.peek(0).unwrap()._type == TokenType::Less {
                self.consume();
                cond.push_str(" < ");
            }
            else if self.peek(0).unwrap()._type == TokenType::Bigger {
                self.consume();
                cond.push_str(" > ");
            }
            else if self.peek(0).unwrap()._type == TokenType::LessEq {
                self.consume();
                cond.push_str(" <= ");
            }
            else if self.peek(0).unwrap()._type == TokenType::BiggerEq {
                self.consume();
                cond.push_str(" >= ");
            }
            else if self.peek(0).unwrap()._type == TokenType::EqEq {
                self.consume();
                cond.push_str(" == ");
            }
            else if self.peek(0).unwrap()._type == TokenType::Diff {
                self.consume();
                cond.push_str(" != ");
            }

            if self.peek(0).unwrap()._type == TokenType::NumberType {
                cond.push_str(self.consume().value.clone().unwrap().as_str());
            }
            else if self.peek(0).unwrap()._type == TokenType::BooleanType {
                cond.push_str(self.consume().value.clone().unwrap().as_str());
            }
            else if self.peek(0).unwrap()._type == TokenType::StringType {
                cond.push_str(self.consume().value.clone().unwrap().as_str());
            }
            else if self.peek(0).unwrap()._type == TokenType::Identifier {
                let ident = self.consume().value.clone().unwrap();

                if !self.m_vars.contains_key(ident.clone().as_str()) {
                    self.print_error(format!("Identifier not exist: {ident}").as_str());
                }

                if let Some(var) = self.m_vars.get(ident.as_str()) {
                    if var._type == TokenType::NumberType {
                        cond.push_str(var.value.clone().as_str());
                    }
                    else if var._type == TokenType::BooleanType {
                        cond.push_str(var.value.clone().as_str());
                    }
                    else if var._type == TokenType::BooleanType {
                        cond.push_str(var.value.clone().as_str());
                    }
                }
            }

            if self.peek(0).unwrap()._type == TokenType::And || self.peek(0).unwrap()._type == TokenType::Or {
                while self.peek(0).is_some() && (self.peek(0).unwrap()._type == TokenType::And || self.peek(0).unwrap()._type == TokenType::Or) {
                    if self.peek(0).unwrap()._type == TokenType::And {
                        self.consume();
                        cond.push_str(" && ");
                    }
                    else if self.peek(0).unwrap()._type == TokenType::Or {
                        self.consume();
                        cond.push_str(" || ");
                    }

                    if self.peek(0).unwrap()._type == TokenType::NumberType {
                        cond.push_str(self.consume().value.clone().unwrap().as_str());
                    }
                    else if self.peek(0).unwrap()._type == TokenType::BooleanType {
                        cond.push_str(self.consume().value.clone().unwrap().as_str());
                    }
                    else if self.peek(0).unwrap()._type == TokenType::StringType {
                        cond.push_str(self.consume().value.clone().unwrap().as_str());
                    }
                    else if self.peek(0).unwrap()._type == TokenType::Identifier {
                        let ident = self.consume().value.clone().unwrap();

                        if !self.m_vars.contains_key(ident.clone().as_str()) {
                            self.print_error(format!("Identifier not exist: {ident}").as_str());
                        }

                        if let Some(var) = self.m_vars.get(ident.as_str()) {
                            if var._type == TokenType::NumberType {
                                cond.push_str(var.value.clone().as_str());
                            }
                            else if var._type == TokenType::BooleanType {
                                cond.push_str(var.value.clone().as_str());
                            }
                            else if var._type == TokenType::BooleanType {
                                cond.push_str(var.value.clone().as_str());
                            }
                        }
                    }
                    else {
                        self.print_error("Expected a condition");
                    }

                    if self.peek(0).unwrap()._type == TokenType::Less {
                        self.consume();
                        cond.push_str(" < ");
                    }
                    else if self.peek(0).unwrap()._type == TokenType::Bigger {
                        self.consume();
                        cond.push_str(" > ");
                    }
                    else if self.peek(0).unwrap()._type == TokenType::LessEq {
                        self.consume();
                        cond.push_str(" <= ");
                    }
                    else if self.peek(0).unwrap()._type == TokenType::BiggerEq {
                        self.consume();
                        cond.push_str(" >= ");
                    }
                    else if self.peek(0).unwrap()._type == TokenType::EqEq {
                        self.consume();
                        cond.push_str(" == ");
                    }
                    else if self.peek(0).unwrap()._type == TokenType::Diff {
                        self.consume();
                        cond.push_str(" != ");
                    }

                    if self.peek(0).unwrap()._type == TokenType::NumberType {
                        cond.push_str(self.consume().value.clone().unwrap().as_str());
                    }
                    else if self.peek(0).unwrap()._type == TokenType::BooleanType {
                        cond.push_str(self.consume().value.clone().unwrap().as_str());
                    }
                    else if self.peek(0).unwrap()._type == TokenType::StringType {
                        cond.push_str(self.consume().value.clone().unwrap().as_str());
                    }
                    else if self.peek(0).unwrap()._type == TokenType::Identifier {
                        let ident = self.consume().value.clone().unwrap();

                        if !self.m_vars.contains_key(ident.clone().as_str()) {
                            self.print_error(format!("Identifier not exist: {ident}").as_str());
                        }

                        if let Some(var) = self.m_vars.get(ident.as_str()) {
                            if var._type == TokenType::NumberType {
                                cond.push_str(var.value.clone().as_str());
                            }
                            else if var._type == TokenType::BooleanType {
                                cond.push_str(var.value.clone().as_str());
                            }
                            else if var._type == TokenType::BooleanType {
                                cond.push_str(var.value.clone().as_str());
                            }
                        }
                    }
                    else {
                        self.print_error("Expected a condition");
                    }
                }
            }

            self.try_consume(TokenType::CloseParam, "Expected `)`");

            //println!("{cond}");

            res = eval(cond.as_str()).unwrap().as_bool().unwrap();

            //println!("{res}");

            self.try_consume(TokenType::OpenCBrackets, "Expected `{`");

            if self.peek(0).is_some() && self.peek(0).unwrap()._type == TokenType::NwLine {
                self.consume();
                //println!("{}", self.line);
                self.line += 1;
            }

            if res {
                while self.peek(0).is_some() && self.peek(0).unwrap()._type != TokenType::CloseCBrackets {
                    if self.peek(0).unwrap()._type == TokenType::CloseCBrackets {
                        break;
                    }
                    self.parse_parse();
                }
            }
            else {
                while self.peek(0).is_some() && self.peek(0).unwrap()._type != TokenType::CloseCBrackets {
                    if self.peek(0).unwrap()._type == TokenType::CloseCBrackets {
                        break;
                    }
                    if self.peek(0).unwrap()._type == TokenType::NwLine {
                        self.line += 1;
                    }
                    self.consume();
                }
            }

            self.try_consume(TokenType::CloseCBrackets, "Expected `}`");
        }

        if self.peek(0).is_some() && self.peek(0).unwrap()._type == TokenType::NwLine {
            self.consume();
            self.line += 1;
            //println!("{}", self.line);
        }

        if self.peek(0).is_some() && self.peek(0).unwrap()._type == TokenType::Else && self.peek(1).is_some() && self.peek(1).unwrap()._type == TokenType::If {
            while self.peek(0).is_some() && self.peek(0).unwrap()._type == TokenType::Else && self.peek(1).is_some() && self.peek(1).unwrap()._type == TokenType::If {
                if !res {
                    self.consume();
                    let mut cond = String::new();
                    self.consume();

                    self.try_consume(TokenType::OpenParam, "Expected `(`");

                    if self.peek(0).unwrap()._type == TokenType::NumberType {
                        cond.push_str(self.consume().value.clone().unwrap().as_str());
                    }
                    else if self.peek(0).unwrap()._type == TokenType::BooleanType {
                        cond.push_str(self.consume().value.clone().unwrap().as_str());
                    }
                    else if self.peek(0).unwrap()._type == TokenType::StringType {
                        cond.push_str(self.consume().value.clone().unwrap().as_str());
                    }
                    else if self.peek(0).unwrap()._type == TokenType::Identifier {
                        let ident = self.consume().value.clone().unwrap();

                        if !self.m_vars.contains_key(ident.clone().as_str()) {
                            self.print_error(format!("Identifier not exist: {ident}").as_str());
                        }

                        if let Some(var) = self.m_vars.get(ident.as_str()) {
                            if var._type == TokenType::NumberType {
                                cond.push_str(var.value.clone().as_str());
                            }
                            else if var._type == TokenType::BooleanType {
                                cond.push_str(var.value.clone().as_str());
                            }
                            else if var._type == TokenType::BooleanType {
                                cond.push_str(var.value.clone().as_str());
                            }
                        }
                    }
                    else {
                        self.print_error("Expected a condition");
                    }

                    if self.peek(0).unwrap()._type == TokenType::Less {
                        self.consume();
                        cond.push_str(" < ");
                    }
                    else if self.peek(0).unwrap()._type == TokenType::Bigger {
                        self.consume();
                        cond.push_str(" > ");
                    }
                    else if self.peek(0).unwrap()._type == TokenType::LessEq {
                        self.consume();
                        cond.push_str(" <= ");
                    }
                    else if self.peek(0).unwrap()._type == TokenType::BiggerEq {
                        self.consume();
                        cond.push_str(" >= ");
                    }
                    else if self.peek(0).unwrap()._type == TokenType::EqEq {
                        self.consume();
                        cond.push_str(" == ");
                    }
                    else if self.peek(0).unwrap()._type == TokenType::Diff {
                        self.consume();
                        cond.push_str(" != ");
                    }

                    if self.peek(0).unwrap()._type == TokenType::NumberType {
                        cond.push_str(self.consume().value.clone().unwrap().as_str());
                    }
                    else if self.peek(0).unwrap()._type == TokenType::BooleanType {
                        cond.push_str(self.consume().value.clone().unwrap().as_str());
                    }
                    else if self.peek(0).unwrap()._type == TokenType::StringType {
                        cond.push_str(self.consume().value.clone().unwrap().as_str());
                    }
                    else if self.peek(0).unwrap()._type == TokenType::Identifier {
                        let ident = self.consume().value.clone().unwrap();

                        if !self.m_vars.contains_key(ident.clone().as_str()) {
                            self.print_error(format!("Identifier not exist: {ident}").as_str());
                        }

                        if let Some(var) = self.m_vars.get(ident.as_str()) {
                            if var._type == TokenType::NumberType {
                                cond.push_str(var.value.clone().as_str());
                            }
                            else if var._type == TokenType::BooleanType {
                                cond.push_str(var.value.clone().as_str());
                            }
                            else if var._type == TokenType::BooleanType {
                                cond.push_str(var.value.clone().as_str());
                            }
                        }
                    }

                    if self.peek(0).unwrap()._type == TokenType::And || self.peek(0).unwrap()._type == TokenType::Or {
                        while self.peek(0).is_some() && (self.peek(0).unwrap()._type == TokenType::And || self.peek(0).unwrap()._type == TokenType::Or) {
                            if self.peek(0).unwrap()._type == TokenType::And {
                                self.consume();
                                cond.push_str(" && ");
                            }
                            else if self.peek(0).unwrap()._type == TokenType::Or {
                                self.consume();
                                cond.push_str(" || ");
                            }

                            if self.peek(0).unwrap()._type == TokenType::NumberType {
                                cond.push_str(self.consume().value.clone().unwrap().as_str());
                            }
                            else if self.peek(0).unwrap()._type == TokenType::BooleanType {
                                cond.push_str(self.consume().value.clone().unwrap().as_str());
                            }
                            else if self.peek(0).unwrap()._type == TokenType::StringType {
                                cond.push_str(self.consume().value.clone().unwrap().as_str());
                            }
                            else if self.peek(0).unwrap()._type == TokenType::Identifier {
                                let ident = self.consume().value.clone().unwrap();

                                if !self.m_vars.contains_key(ident.clone().as_str()) {
                                    self.print_error(format!("Identifier not exist: {ident}").as_str());
                                }

                                if let Some(var) = self.m_vars.get(ident.as_str()) {
                                    if var._type == TokenType::NumberType {
                                        cond.push_str(var.value.clone().as_str());
                                    }
                                    else if var._type == TokenType::BooleanType {
                                        cond.push_str(var.value.clone().as_str());
                                    }
                                    else if var._type == TokenType::BooleanType {
                                        cond.push_str(var.value.clone().as_str());
                                    }
                                }
                            }
                            else {
                                self.print_error("Expected a condition");
                            }

                            if self.peek(0).unwrap()._type == TokenType::Less {
                                self.consume();
                                cond.push_str(" < ");
                            }
                            else if self.peek(0).unwrap()._type == TokenType::Bigger {
                                self.consume();
                                cond.push_str(" > ");
                            }
                            else if self.peek(0).unwrap()._type == TokenType::LessEq {
                                self.consume();
                                cond.push_str(" <= ");
                            }
                            else if self.peek(0).unwrap()._type == TokenType::BiggerEq {
                                self.consume();
                                cond.push_str(" >= ");
                            }
                            else if self.peek(0).unwrap()._type == TokenType::EqEq {
                                self.consume();
                                cond.push_str(" == ");
                            }
                            else if self.peek(0).unwrap()._type == TokenType::Diff {
                                self.consume();
                                cond.push_str(" != ");
                            }

                            if self.peek(0).unwrap()._type == TokenType::NumberType {
                                cond.push_str(self.consume().value.clone().unwrap().as_str());
                            }
                            else if self.peek(0).unwrap()._type == TokenType::BooleanType {
                                cond.push_str(self.consume().value.clone().unwrap().as_str());
                            }
                            else if self.peek(0).unwrap()._type == TokenType::StringType {
                                cond.push_str(self.consume().value.clone().unwrap().as_str());
                            }
                            else if self.peek(0).unwrap()._type == TokenType::Identifier {
                                let ident = self.consume().value.clone().unwrap();

                                if !self.m_vars.contains_key(ident.clone().as_str()) {
                                    self.print_error(format!("Identifier not exist: {ident}").as_str());
                                }

                                if let Some(var) = self.m_vars.get(ident.as_str()) {
                                    if var._type == TokenType::NumberType {
                                        cond.push_str(var.value.clone().as_str());
                                    }
                                    else if var._type == TokenType::BooleanType {
                                        cond.push_str(var.value.clone().as_str());
                                    }
                                    else if var._type == TokenType::BooleanType {
                                        cond.push_str(var.value.clone().as_str());
                                    }
                                }
                            }
                            else {
                                self.print_error("Expected a condition");
                            }
                        }
                    }

                    self.try_consume(TokenType::CloseParam, "Expected `)`");

                    //println!("{cond}");

                    res = eval(cond.as_str()).unwrap().as_bool().unwrap();

                    //println!("{res}");

                    self.try_consume(TokenType::OpenCBrackets, "Expected `{`");

                    if self.peek(0).is_some() && self.peek(0).unwrap()._type == TokenType::NwLine {
                        self.consume();
                        //println!("{}", self.line);
                        self.line += 1;
                    }

                    if res {
                        while self.peek(0).is_some() && self.peek(0).unwrap()._type != TokenType::CloseCBrackets {
                            if self.peek(0).unwrap()._type == TokenType::CloseCBrackets {
                                break;
                            }
                            self.parse_parse();
                        }
                    }
                    else {
                        while self.peek(0).is_some() && self.peek(0).unwrap()._type != TokenType::CloseCBrackets {
                            if self.peek(0).unwrap()._type == TokenType::CloseCBrackets {
                                break;
                            }
                            if self.peek(0).unwrap()._type == TokenType::NwLine {
                                self.line += 1;
                            }
                            self.consume();
                        }
                    }

                    self.try_consume(TokenType::CloseCBrackets, "Expected `}`");

                    if self.peek(0).is_some() && self.peek(0).unwrap()._type == TokenType::NwLine {
                        self.consume();
                        self.line += 1;
                    }
                }
            }
        }

        if self.peek(0).is_some() && self.peek(0).unwrap()._type == TokenType::NwLine {
            self.consume();
            self.line += 1;
        }

        if self.peek(0).is_some() && self.peek(0).unwrap()._type == TokenType::Else {
            self.consume();
            self.try_consume(TokenType::OpenCBrackets, "Expected `{`");

            if !res {
                while self.peek(0).is_some() && self.peek(0).unwrap()._type != TokenType::CloseCBrackets {
                    if self.peek(0).unwrap()._type == TokenType::CloseCBrackets {
                        break;
                    }
                    self.parse_parse();
                }
            }
            else {
                while self.peek(0).is_some() && self.peek(0).unwrap()._type != TokenType::CloseCBrackets {
                    if self.peek(0).unwrap()._type == TokenType::CloseCBrackets {
                        break;
                    }
                    self.consume();
                }
            }

            self.try_consume(TokenType::CloseCBrackets, "Expected `}`");
        }
    }

    fn parse_while(&mut self) {
        let mut res = true;
        let s_index = self.index;
        let mut l_index: usize = 0;
        let mut end_of_while = false;

        while res {
            self.index = s_index;
            let mut cond = String::new();

            self.try_consume(TokenType::OpenParam, "Expected `(`");

            if self.peek(0).unwrap()._type == TokenType::NumberType {
                cond.push_str(self.consume().value.clone().unwrap().as_str());
            }
            else if self.peek(0).unwrap()._type == TokenType::BooleanType {
                cond.push_str(self.consume().value.clone().unwrap().as_str());
            }
            else if self.peek(0).unwrap()._type == TokenType::StringType {
                cond.push_str(self.consume().value.clone().unwrap().as_str());
            }
            else if self.peek(0).unwrap()._type == TokenType::Identifier {
                let ident = self.consume().value.clone().unwrap();

                if !self.m_vars.contains_key(ident.clone().as_str()) {
                    self.print_error(format!("Identifier not exist: {ident}").as_str());
                }

                if let Some(var) = self.m_vars.get(ident.as_str()) {
                    if var._type == TokenType::NumberType {
                        cond.push_str(var.value.clone().as_str());
                    } else if var._type == TokenType::BooleanType {
                        cond.push_str(var.value.clone().as_str());
                    } else if var._type == TokenType::BooleanType {
                        cond.push_str(var.value.clone().as_str());
                    }
                }
            }
            else {
                self.print_error("Expected a condition");
            }

            if self.peek(0).unwrap()._type == TokenType::Less {
                self.consume();
                cond.push_str(" < ");
            }
            else if self.peek(0).unwrap()._type == TokenType::Bigger {
                self.consume();
                cond.push_str(" > ");
            }
            else if self.peek(0).unwrap()._type == TokenType::LessEq {
                self.consume();
                cond.push_str(" <= ");
            }
            else if self.peek(0).unwrap()._type == TokenType::BiggerEq {
                self.consume();
                cond.push_str(" >= ");
            }
            else if self.peek(0).unwrap()._type == TokenType::EqEq {
                self.consume();
                cond.push_str(" == ");
            }
            else if self.peek(0).unwrap()._type == TokenType::Diff {
                self.consume();
                cond.push_str(" != ");
            }

            if self.peek(0).unwrap()._type == TokenType::NumberType {
                cond.push_str(self.consume().value.clone().unwrap().as_str());
            }
            else if self.peek(0).unwrap()._type == TokenType::BooleanType {
                cond.push_str(self.consume().value.clone().unwrap().as_str());
            }
            else if self.peek(0).unwrap()._type == TokenType::StringType {
                cond.push_str(self.consume().value.clone().unwrap().as_str());
            }
            else if self.peek(0).unwrap()._type == TokenType::Identifier {
                let ident = self.consume().value.clone().unwrap();

                if !self.m_vars.contains_key(ident.clone().as_str()) {
                    self.print_error(format!("Identifier not exist: {ident}").as_str());
                }

                if let Some(var) = self.m_vars.get(ident.as_str()) {
                    if var._type == TokenType::NumberType {
                        cond.push_str(var.value.clone().as_str());
                    } else if var._type == TokenType::BooleanType {
                        cond.push_str(var.value.clone().as_str());
                    } else if var._type == TokenType::BooleanType {
                        cond.push_str(var.value.clone().as_str());
                    }
                }
            }
            else {
                self.print_error("Expected a condition");
            }

            if self.peek(0).unwrap()._type == TokenType::And || self.peek(0).unwrap()._type == TokenType::Or {
                while self.peek(0).is_some() && (self.peek(0).unwrap()._type == TokenType::And || self.peek(0).unwrap()._type == TokenType::Or) {
                    if self.peek(0).unwrap()._type == TokenType::And {
                        self.consume();
                        cond.push_str(" && ");
                    }
                    else if self.peek(0).unwrap()._type == TokenType::Or {
                        self.consume();
                        cond.push_str(" || ");
                    }

                    if self.peek(0).unwrap()._type == TokenType::NumberType {
                        cond.push_str(self.consume().value.clone().unwrap().as_str());
                    }
                    else if self.peek(0).unwrap()._type == TokenType::BooleanType {
                        cond.push_str(self.consume().value.clone().unwrap().as_str());
                    }
                    else if self.peek(0).unwrap()._type == TokenType::StringType {
                        cond.push_str(self.consume().value.clone().unwrap().as_str());
                    }
                    else if self.peek(0).unwrap()._type == TokenType::Identifier {
                        let ident = self.consume().value.clone().unwrap();

                        if !self.m_vars.contains_key(ident.clone().as_str()) {
                            self.print_error(format!("Identifier not exist: {ident}").as_str());
                        }

                        if let Some(var) = self.m_vars.get(ident.as_str()) {
                            if var._type == TokenType::NumberType {
                                cond.push_str(var.value.clone().as_str());
                            } else if var._type == TokenType::BooleanType {
                                cond.push_str(var.value.clone().as_str());
                            } else if var._type == TokenType::BooleanType {
                                cond.push_str(var.value.clone().as_str());
                            }
                        }
                    }
                    else {
                        self.print_error("Expected a condition");
                    }

                    if self.peek(0).unwrap()._type == TokenType::Less {
                        self.consume();
                        cond.push_str(" < ");
                    }
                    else if self.peek(0).unwrap()._type == TokenType::Bigger {
                        self.consume();
                        cond.push_str(" > ");
                    }
                    else if self.peek(0).unwrap()._type == TokenType::LessEq {
                        self.consume();
                        cond.push_str(" <= ");
                    }
                    else if self.peek(0).unwrap()._type == TokenType::BiggerEq {
                        self.consume();
                        cond.push_str(" >= ");
                    }
                    else if self.peek(0).unwrap()._type == TokenType::EqEq {
                        self.consume();
                        cond.push_str(" == ");
                    }
                    else if self.peek(0).unwrap()._type == TokenType::Diff {
                        self.consume();
                        cond.push_str(" != ");
                    }

                    if self.peek(0).unwrap()._type == TokenType::NumberType {
                        cond.push_str(self.consume().value.clone().unwrap().as_str());
                    }
                    else if self.peek(0).unwrap()._type == TokenType::BooleanType {
                        cond.push_str(self.consume().value.clone().unwrap().as_str());
                    }
                    else if self.peek(0).unwrap()._type == TokenType::StringType {
                        cond.push_str(self.consume().value.clone().unwrap().as_str());
                    }
                    else if self.peek(0).unwrap()._type == TokenType::Identifier {
                        let ident = self.consume().value.clone().unwrap();

                        if !self.m_vars.contains_key(ident.clone().as_str()) {
                            self.print_error(format!("Identifier not exist: {ident}").as_str());
                        }

                        if let Some(var) = self.m_vars.get(ident.as_str()) {
                            if var._type == TokenType::NumberType {
                                cond.push_str(var.value.clone().as_str());
                            } else if var._type == TokenType::BooleanType {
                                cond.push_str(var.value.clone().as_str());
                            } else if var._type == TokenType::BooleanType {
                                cond.push_str(var.value.clone().as_str());
                            }
                        }
                    }
                    else {
                        self.print_error("Expected a condition");
                    }
                }
            }

            self.try_consume(TokenType::CloseParam, "Expected `)`");

            //println!("{cond}");

            res = eval(cond.as_str()).unwrap().as_bool().unwrap();

            //println!("{res}");

            self.try_consume(TokenType::OpenCBrackets, "Expected `{`");

            if self.peek(0).is_some() && self.peek(0).unwrap()._type == TokenType::NwLine {
                self.consume();
                //println!("{}", self.line);
                if !end_of_while {
                    self.line += 1;
                }
            }

            if res {
                while self.peek(0).is_some() && self.peek(0).unwrap()._type != TokenType::CloseCBrackets {
                    if self.peek(0).unwrap()._type == TokenType::CloseCBrackets {
                        break;
                    }
                    self.parse_parse_w(end_of_while);
                }
            }
            else {
                while self.peek(0).is_some() && self.peek(0).unwrap()._type != TokenType::CloseCBrackets {
                    if self.peek(0).unwrap()._type == TokenType::CloseCBrackets {
                        break;
                    }
                    if self.peek(0).unwrap()._type == TokenType::NwLine {
                        self.consume();
                        //println!("{}", self.line);
                        if !end_of_while {
                            self.line += 1;
                        }
                    }
                    self.consume();
                }
            }

            if !end_of_while {
                l_index = self.index;
            }

            end_of_while = true;
        }

        self.index = l_index;

        self.try_consume(TokenType::CloseCBrackets, "Expected `}`")
    }

    fn parse_for(&mut self) {
        let mut ident = String::new();

        self.try_consume(TokenType::OpenParam, "Expected `(`");

        if self.peek(0).unwrap()._type == TokenType::Var {
            self.consume();
            if self.peek(0).unwrap()._type == TokenType::Identifier {
                ident = self.consume().value.clone().unwrap();
            }

            self.try_consume(TokenType::Equals, "Expected `=`");

            if self.peek(0).unwrap()._type == TokenType::NumberType {
                let value = self.consume().value.clone().unwrap();

                self.m_vars.insert(ident.clone(), Vars {_type: TokenType::NumberType, value});

                println!("{:?}", self.m_vars);
            }
            else {
                self.print_error("Invalid value type. Expected a numeric value");
            }

            self.try_consume(TokenType::Semi, "Expected `;`");
        }

        let s_index = self.index;
        let mut l_index = 0;
        let mut res = true;
        let mut op = ' ';
        let mut end_of_for = false;

        while res {
            self.index = s_index;
            let mut cond = String::new();

            if self.peek(0).unwrap()._type == TokenType::Identifier {
                let n_ident = self.consume().value.clone().unwrap();

                if n_ident != ident.clone() {
                    self.print_error("Use the for variable");
                }

                if let Some(var) = self.m_vars.get(n_ident.as_str()) {
                    if var._type == TokenType::NumberType {
                        cond.push_str(var.value.as_str());
                    }
                }
            }
            else {
                self.print_error("Expected a condition");
            }

            if self.peek(0).unwrap()._type == TokenType::Less {
                self.consume();
                cond.push_str(" < ");
            }
            else if self.peek(0).unwrap()._type == TokenType::Bigger {
                self.consume();
                cond.push_str(" > ");
            }
            else if self.peek(0).unwrap()._type == TokenType::LessEq {
                self.consume();
                cond.push_str(" <= ");
            }
            else if self.peek(0).unwrap()._type == TokenType::BiggerEq {
                self.consume();
                cond.push_str(" >= ");
            }
            else if self.peek(0).unwrap()._type == TokenType::EqEq {
                self.consume();
                cond.push_str(" == ");
            }
            else if self.peek(0).unwrap()._type == TokenType::Diff {
                self.consume();
                cond.push_str(" != ");
            }

            if self.peek(0).unwrap()._type == TokenType::NumberType {
                cond.push_str(self.consume().value.clone().unwrap().as_str());
            }
            else {
                self.print_error("Invalid value type. Expected a numeric value");
            }

            println!("{cond}");

            res = eval(cond.as_str()).unwrap().as_bool().unwrap();

            println!("{res}");

            self.try_consume(TokenType::Semi, "Expected `;`");

            if self.peek(0).unwrap()._type == TokenType::Identifier {
                let n_ident = self.consume().value.clone().unwrap();

                if n_ident != ident.clone() {
                    self.print_error("Use the for variable");
                }
            }

            if self.peek(0).unwrap()._type == TokenType::PPlus {
                self.consume();
                op = '+';
            }
            else if self.peek(0).unwrap()._type == TokenType::MMinus {
                self.consume();
                op = '-';
            }
            
            self.try_consume(TokenType::CloseParam, "Expected `)`");
            self.try_consume(TokenType::OpenCBrackets, "Expected `{`");

            if self.peek(0).is_some() && self.peek(0).unwrap()._type == TokenType::NwLine {
                self.consume();
                //println!("{}", self.line);
                if !end_of_for {
                    self.line += 1;
                }
            }

            if res {
                while self.peek(0).is_some() && self.peek(0).unwrap()._type != TokenType::CloseCBrackets {
                    if self.peek(0).unwrap()._type == TokenType::CloseCBrackets {
                        break;
                    }
                    self.parse_parse_w(end_of_for);
                }
            }
            else {
                while self.peek(0).is_some() && self.peek(0).unwrap()._type != TokenType::CloseCBrackets {
                    if self.peek(0).unwrap()._type == TokenType::CloseCBrackets {
                        break;
                    }
                    if self.peek(0).unwrap()._type == TokenType::NwLine {
                        self.consume();
                        //println!("{}", self.line);
                        if !end_of_for {
                            self.line += 1;
                        }
                    }
                    self.consume();
                }
            }

            if !end_of_for {
                l_index = self.index;
            }

            if let Some(var) = self.m_vars.get(ident.as_str()) {
                let mut expr = String::new();
                if var._type == TokenType::NumberType {
                    expr.push_str(var.value.as_str());

                    if op == '+' {
                        expr.push_str("+1");
                    }
                    else if op == '-' {
                        expr.push_str("-1");
                    }
                }

                let v = eval(expr.as_str()).unwrap();

                self.m_vars.insert(ident.clone(), Vars {_type: TokenType::NumberType, value: v.to_string()});
            }

            end_of_for = true;
        }
        self.index = l_index;

        self.try_consume(TokenType::CloseCBrackets, "Expected `}`");

        self.m_vars.remove(ident.as_str());
    }

    pub fn parse(&mut self) {
        while self.peek(0).is_some() {
            if self.peek(0).unwrap()._type == TokenType::Print {
                self.consume();
                let contents = self.parse_print();
                println!("{contents}");
            }
            else if self.peek(0).unwrap()._type == TokenType::Var {
                self.parse_vars();
            }
            else if self.peek(0).unwrap()._type == TokenType::Identifier {
                self.parse_vars();
            }
            else if self.peek(0).unwrap()._type == TokenType::If {
                self.parse_if();
            }
            else if self.peek(0).unwrap()._type == TokenType::While {
                self.consume();
                self.parse_while();
            }
            else if self.peek(0).unwrap()._type == TokenType::For {
                self.consume();
                self.parse_for();
            }
            else if self.peek(0).unwrap()._type == TokenType::NwLine {
                self.consume();
                self.line += 1;
            }
        }
    }

    fn print_error(&mut self, msg: &str) {
        eprintln!("Line: {}, Col {}: {}", self.line, self.c_col, msg);
        exit(101);
    }

    fn peek(&mut self, offset: usize) -> Option<&Token> {
        self.m_tokens.iter().nth(self.index + offset)
    }

    fn try_consume(&mut self, _type: TokenType, msg: &str) {
        if self.peek(0).is_some() && self.peek(0).unwrap()._type == _type {
            self.consume();
        }
        else {
            self.print_error(format!("{}", msg).as_str());
            exit(101);
        }
    }

    fn consume(&mut self) -> &Token {
        self.index += 1;
        self.c_col = self.m_tokens.iter().nth(self.index - 1).unwrap().col.clone();
        //println!("C_C: {}", self.c_col);
        self.m_tokens.iter().nth(self.index - 1).unwrap()
    }
}
