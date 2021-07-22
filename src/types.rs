use std::{error::Error, num};

/// represents a command such as `pop local 2`
#[derive(Debug)]
pub struct Command {
    pub action: Action,
    pub segment: Segment,
    pub address: u16,
}

/// An arithmetic or logical operation
#[derive(Debug)]
pub enum Arithmetic {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

/// Represents a memory segement
#[derive(Debug)]
pub enum Segment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

/// represents a push or pop!
#[derive(Debug)]
pub enum Action {
    Push,
    Pop,
}

/// represents a flow command
/// example: `label YO_LABEL`
/// example: `if-goto YO_LABEL`
/// example: `goto YO_LABEL`
#[derive(Debug)]
pub struct FlowControl<'a> {
    pub flow_type: FlowType,
    pub label: &'a str,
}

#[derive(Debug)]
pub enum FlowType {
    Label,
    Goto,
    IfGoto,
}

/// Function
/// example: function mult 0
#[derive(Debug)]
pub struct Function<'a> {
    pub name: &'a str,
    pub local_vars: u16,
}

/// Function Call,
/// example: call mult 0
#[derive(Debug)]
pub struct FunctionCall<'a> {
    pub name: &'a str,
    pub args: u16,
}

/// parsed line
#[derive(Debug)]
pub enum ParsedLine<'a> {
    /// example: pop local 3
    Command(Command),
    /// example: add
    Arithmetic(Arithmetic),
    /// example: label YO_LABEL
    /// example: if-goto YO_LABEL
    FlowControl(FlowControl<'a>),
    /// a return statment
    Return,
    /// Function
    /// example: function mult 0
    Function(Function<'a>),
    /// Function Call
    /// example: call mult 0
    FunctionCall(FunctionCall<'a>),
}

#[derive(Debug)]
pub struct ParseError(pub String);
impl Error for ParseError {}

impl From<num::ParseIntError> for ParseError {
    fn from(error: num::ParseIntError) -> Self {
        ParseError(format!("Error from num::ParseIntError {}", error))
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
