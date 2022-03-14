use crate::token::Token;
use crate::error;
use crate::value::Value;

use logos::Lexer;
use colored::*;

use std::fmt::Formatter;
use std::fmt::Display;
use std::collections::HashMap;

// used to make instructions and instruction groups work the same
pub trait InstructionCompiler {
    fn get_token(&self) -> Token;
    fn compile(&self, lex: &mut Lexer<Token>, variables: &[String], labels: &HashMap<String, u32>, routines: &HashMap<String, u32>, filename: &str, line: u32, debug: bool, current_routine: &Option<String>, num_instructions: usize) -> Vec<String>; // like above, turns a value for a command from tokens back into text, double checking the arguments
}

pub struct Instruction {
    pub name: String,
    pub token: Token,
    pub arguments: Vec<Value>,
    pub super_instruction_name: Option<String>,
}

impl InstructionCompiler for Instruction {
    fn get_token(&self) -> Token {
        self.token.clone()
    }

    fn compile(&self, lex: &mut Lexer<Token>, variables: &[String], _labels: &HashMap<String, u32>, _routines: &HashMap<String, u32>, filename: &str, line: u32, _debug: bool, _current_routine: &Option<String>, _num_instructions: usize) -> Vec<String> {
        let mut elements = vec![self.name.to_string()];
        for arg in self.arguments.iter() {
            if let Some(compiled) = arg.compile(lex, variables, false, filename, line) {
                elements.push(compiled);
            } else {
                println!("{} instruction is defined as: {}", "note:".bold(), self);
                std::process::exit(1);
            }
        }
        vec![elements.join(" ")]
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.super_instruction_name {
            write!(f, "{} {}", name, self.name)?;
        } else {
            write!(f, "{}", self.name)?;
        }
        //write!(f, );
        for arg in self.arguments.iter() {
            write!(f, " {}", arg.to_string_with_name())?;
        }
        Ok(())
    }
}

pub struct InstructionGroup { // for those Wacky Bois with sub instructions
    pub name: String,
    pub token: Token,
    pub sub_instructions: Vec<Instruction>
}

impl InstructionCompiler for InstructionGroup {
    fn get_token(&self) -> Token {
        self.token.clone()
    }

    fn compile(&self, lex: &mut Lexer<Token>, variables: &[String], labels: &HashMap<String, u32>, routines: &HashMap<String, u32>, filename: &str, line: u32, debug: bool, current_routine: &Option<String>, num_instructions: usize) -> Vec<String> {
        let mut elements = vec![self.name.to_string()];
        let next = lex.next();
        match next {
            Some(Token::Newline) => error(&format!("unexpected newline after instruction \"{}\"", self.name), filename, line),
            Some(token) => {
                let mut has_inst = false;
                for inst in self.sub_instructions.iter() {
                    if inst.get_token() == token {
                        elements.append(&mut inst.compile(lex, variables, labels, routines, filename, line, debug, current_routine, num_instructions));
                        has_inst = true;
                        break;
                    }
                }
                if !has_inst {
                    error(&format!("unknown sub-instruction {:?} for \"{}\"", token, self.name), filename, line);
                }
            },
            None => error("expected sub-instruction name, got EOF", filename, line),
        }
        vec![elements.join(" ")]
    }
}

pub struct InstructionJump {}

impl InstructionCompiler for InstructionJump {
    fn get_token(&self) -> Token {
        Token::InstJump
    }

    fn compile(&self, lex: &mut Lexer<Token>, variables: &[String], labels: &HashMap<String, u32>, _routines: &HashMap<String, u32>, filename: &str, line: u32, debug: bool, _current_routine: &Option<String>, _num_instructions: usize) -> Vec<String> {
        fn print_def() {
            println!("{} instruction is defined as: jump label (label) comp (Comp) a (any) b (any)", "note:".bold());
            println!("or: jump label (label) always");
        }

        let mut elements = vec!["jump".to_string()];
        let label_name = match lex.next() {
            Some(Token::Name(label)) => {
                if let Some(pos) = labels.get(&label) {
                    elements.push(format!("{}", pos));
                    label
                } else {
                    error(&format!("couldn't find label {:?}", label), filename, line);
                    std::process::exit(1);
                }
            },
            err => {
                error(&format!("expected label, got {:?}", err), filename, line);
                print_def();
                std::process::exit(1);
            }
        };
        match lex.next() {
            Some(Token::Op(comp)) => {
                elements.push(comp);
                if let Some(compiled) = Value::Any("a".to_string()).compile(lex, variables, false, filename, line) {
                    elements.push(compiled);
                } else {
                    std::process::exit(1);
                }
                if let Some(compiled) = Value::Any("b".to_string()).compile(lex, variables, true, filename, line) {
                    elements.push(compiled);
                } else {
                    std::process::exit(1);
                }
            },
            Some(Token::Always) => elements.push("always".to_string()),
            err => {
                error(&format!("expected comparison, got {:?}", err), filename, line);
                print_def();
                std::process::exit(1);
            }
        }
        if debug {
            elements.push(format!("# jump to {}", label_name));
        }

        vec![elements.join(" ")]
    }
}

pub struct InstructionGoto {}

impl InstructionCompiler for InstructionGoto {
    fn get_token(&self) -> Token {
        Token::InstGoto
    }

    fn compile(&self, lex: &mut Lexer<Token>, _variables: &[String], labels: &HashMap<String, u32>, routines: &HashMap<String, u32>, filename: &str, line: u32, debug: bool, _current_routine: &Option<String>, _num_instructions: usize) -> Vec<String> {
        let mut elements = vec!["jump".to_string()];
        match lex.next() {
            Some(Token::Name(label)) => {
                if let Some(pos) = labels.get(&label) {
                    elements.push(format!("{} always", pos));
                    if debug {
                        elements.push(format!("# goto {}", label));
                    }
                } else {
                    if routines.contains_key(&label) {
                        error(&format!("{} is declared as a subroutine, not a label!", label), filename, line);
                    } else {
                        error(&format!("couldn't find label {}", label), filename, line);
                    }
                    std::process::exit(1);
                }
            },
            err => {
                error(&format!("expected label, got {:?}", err), filename, line);
                println!("{} instruction is defined as: goto label (label)", "note:".bold());
                std::process::exit(1);
            }
        }

        vec![elements.join(" ")]
    }
}

pub struct InstructionOp {}

impl InstructionCompiler for InstructionOp {
    fn get_token(&self) -> Token {
        Token::InstOp
    }

    fn compile(&self, lex: &mut Lexer<Token>, variables: &[String], _labels: &HashMap<String, u32>, _routines: &HashMap<String, u32>, filename: &str, line: u32, _debug: bool, _current_routine: &Option<String>, _num_instructions: usize) -> Vec<String> {
        fn print_def() {
            println!("{} instruction is defined as: op op (op) result (variable) a (any) b (any, optional)", "note:".bold());
        }

        let mut elements = vec!["op".to_string()];
        match lex.next() {
            Some(Token::Op(op)) => elements.push(op),
            err => {
                error(&format!("expected op, got {:?}", err), filename, line);
                print_def();
                std::process::exit(1);
            }
        }
        if let Some(compiled) = Value::Variable("result".to_string()).compile(lex, variables, false, filename, line) {
            elements.push(compiled);
        } else {
            print_def();
            std::process::exit(1);
        }
        if let Some(compiled) = Value::Any("a".to_string()).compile(lex, variables, false, filename, line) {
            elements.push(compiled);
        } else {
            print_def();
            std::process::exit(1);
        }
        if let Some(compiled) = Value::Any("b".to_string()).compile(lex, variables, true, filename, line) {
            elements.push(compiled);
        } else {
            print_def();
            std::process::exit(1);
        }

        vec![elements.join(" ")]
    }
}

pub struct InstructionGosub {}

impl InstructionCompiler for InstructionGosub {
    fn get_token(&self) -> Token {
        Token::InstGosub
    }

    fn compile(&self, lex: &mut Lexer<Token>, _variables: &[String], _labels: &HashMap<String, u32>, routines: &HashMap<String, u32>, filename: &str, line: u32, debug: bool, current_routine: &Option<String>, num_instructions: usize) -> Vec<String> {
        let routine_name = match lex.next() {
            Some(Token::Name(label)) => label,
            err => {
                error(&format!("expected routine, got {:?}", err), filename, line);
                println!("{} instruction is defined as: gosub routine (routine)", "note:".bold());
                std::process::exit(1);
            }
        };

        if let Some(current) = current_routine {
            if &routine_name == current {
                error("can't call the current subroutine!", filename, line);
                std::process::exit(1);
            }
        }

        let mut elements = vec!["jump".to_string()];
        if let Some(pos) = routines.get(&routine_name) {
            elements.push(format!("{}", pos));
        } else {
            error(&format!("couldn't find routine {}", routine_name), filename, line);
            std::process::exit(1);
        }
        elements.push("always".to_string());
        if debug {
            elements.push(format!("# gosub {}", routine_name));
        }

        vec![format!("set {}Return {}", routine_name, num_instructions + 2), elements.join(" ")]
    }
}
