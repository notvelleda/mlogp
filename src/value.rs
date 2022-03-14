use logos::Lexer;

use crate::token::Token;
use crate::misc::{error, error_no_line};

#[derive(Debug, PartialEq)]
pub enum Value {
    Bool(String), // string is name of value
    Float(String),
    Int(String),
    String(String),
    Name(String, String),
    Variable(String),
    Any(String),
}

// will turn a value for a command from tokens back into text, double checking the arguments
impl Value {
    pub fn compile(&self, lex: &mut Lexer<Token>, variables: &[String], allow_none: bool, filename: &str, line: u32) -> Option<String> {
        let next = lex.next();
        match &next {
            None => {
                if !allow_none {
                    error_no_line(&format!("expected {} or variable, got EOF", self.to_string()), filename)
                }
                None
            },
            Some(Token::Newline) => {
                if !allow_none {
                    error(&format!("expected {} or variable, got {} (maybe you forgot to create a variable?)", self.to_string(), lex.slice()), filename, line)
                }
                None
            },
            Some(Token::Bool(val)) => {
                match self {
                    Value::Bool(_) | Value::Any(_) => Some(val.to_string()),
                    _ => {
                        error(&format!("expected {} or variable, got {} (maybe you forgot to create a variable?)", self.to_string(), lex.slice()), filename, line);
                        None
                    },
                }
            },
            Some(Token::Float(val)) => {
                match self {
                    Value::Float(_) | Value::Any(_) => Some(val.to_string()),
                    _ => {
                        error(&format!("expected {} or variable, got {} (maybe you forgot to create a variable?)", self.to_string(), lex.slice()), filename, line);
                        None
                    },
                }
            },
            Some(Token::Int(val)) => {
                match self {
                    Value::Int(_) | Value::Any(_) => Some(val.to_string()),
                    _ => {
                        error(&format!("expected {} or variable, got {} (maybe you forgot to create a variable?)", self.to_string(), lex.slice()), filename, line);
                        None
                    },
                }
            },
            Some(Token::String(val)) => {
                match self {
                    Value::String(_) | Value::Any(_) => Some(val.to_string()),
                    _ => {
                        error(&format!("expected {} or variable, got {} (maybe you forgot to create a variable?)", self.to_string(), lex.slice()), filename, line);
                        None
                    },
                }
            },
            Some(Token::Name(val)) => {
                if variables.contains(val) {
                    Some(val.to_string())
                } else {
                    match self {
                        Value::Name(_, _) | Value::Any(_) => Some(val.to_string()),
                        _ => {
                            error(&format!("expected {} or variable, got {} (maybe you forgot to create a variable?)", self.to_string(), lex.slice()), filename, line);
                            None
                        },
                    }
                }
            },
            Some(Token::SysVar(val)) => Some(val.to_string()),
            _ => {
                error(&format!("expected {} or variable, got {} (maybe you forgot to create a variable?)", self.to_string(), lex.slice()), filename, line);
                None
            },
        }
    }

    pub fn to_string_with_name(&self) -> String {
        match self {
            Value::Bool(name) => format!("{} (bool)", name),
            Value::Float(name) => format!("{} (float)", name),
            Value::Int(name) => format!("{} (int)", name),
            Value::String(name) => format!("{} (string)", name),
            Value::Name(name, name2) => format!("{} (name ({}))", name, name2),
            Value::Variable(name) => format!("{} (variable)", name),
            Value::Any(name) => format!("{} (any)", name),
        }
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Bool(_) => "bool".to_string(),
            Value::Float(_) => "float".to_string(),
            Value::Int(_) => "int".to_string(),
            Value::String(_) => "string".to_string(),
            Value::Name(_, name) => format!("name ({})", name),
            Value::Variable(_) => "variable".to_string(),
            Value::Any(_) => "any".to_string(),
        }
    }
}
