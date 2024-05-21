#[derive(Debug, PartialEq)]
pub enum TokenType {
    Print,
    Input,
    OpenParam,
    CloseParam,
    StringType,
    NumberType,
    BooleanType,
    Var,
    Identifier,
    Add,
    PPlus,
    Minus,
    MMinus,
    Star,
    Div,
    Equals,
    Dot,
    Number,
    Boolean,
    If,
    Else,
    While,
    For,
    OpenCBrackets,
    CloseCBrackets,
    Less,
    LessEq,
    Bigger,
    BiggerEq,
    EqEq,
    Diff,
    And,
    Or,
    NwLine,
    Semi
}

#[derive(Debug)]
pub struct Token {
    pub _type: TokenType,
    pub value: Option<String>,
    pub col: usize
}
