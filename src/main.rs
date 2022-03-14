use std::fs;
use std::fs::File;
use std::collections::HashMap;
use std::io::Write;

use clap::Parser;
use logos::Logos;

pub mod token;
pub mod instruction_compiler;
pub mod instruction_list;
pub mod value;
pub mod misc;

use token::Token;
use misc::error;
use instruction_list::create_instructions;

// command line arguments
#[derive(Parser, Debug)]
#[clap(author = "velleda", version, about = "very slightly higher level wrapper around mlog", long_about = "very slightly higher level wrapper around mlog. makes jump statements actually usable and will yell at you incomprehensibly if you make a mistake")]
pub struct Args {
    /// Input file
    #[clap(short, long)]
    input: String,

    /// Output file (setting to stdout will send output to stdout)
    #[clap(short, long, default_value = "stdout")]
    output: String,

    /// Add debug comments to generated code
    #[clap(short, long)]
    debugcomments: bool,
}

pub fn main() {
    let args = Args::parse();
    //println!("{:?}", args);
    let string = fs::read_to_string(&args.input).unwrap();

    // first pass: find label names and locations, variables
    let mut lex = Token::lexer(&string);
    let mut inst_counter = 0;
    let mut line_count = 1;
    let mut labels: HashMap<String, u32> = HashMap::new();
    let mut routines: HashMap<String, u32> = HashMap::new();
    let mut variables: Vec<String> = Vec::new();

    while let Some(token) = lex.next() {
        //println!("pass 1 {:?}", token);
        let mut skip_to_next = true;
        match token {
            Token::InstWrite | Token::InstDraw | Token::InstDrawFlush | 
            Token::InstPrint | Token::InstPrintFlush | Token::InstControl |
            Token::InstEnd | Token::InstJump | Token::InstUnitBind |
            Token::InstUnitRadar | Token::InstNoOp | Token::InstGoto |
            Token::InstReturn => inst_counter += 1, // emits one instruction

            Token::InstGosub | Token::InstGosubCond => inst_counter += 2, // emits two instructions

            Token::InstRead | Token::InstGetLink | Token::InstSensor |
            Token::InstSet => {
                inst_counter += 1;
                //println!("token {:?} has var", token);
                match lex.next() {
                    Some(Token::Name(var)) => {
                        if !variables.contains(&var) {
                            //println!("token {:?}, var {}", token, var);
                            variables.push(var);
                        }
                    },
                    Some(Token::SysVar(_)) => (),
                    Some(unx) => error(&format!("unexpected token {:?} after {:?}", unx, token), &args.input, line_count),
                    None => error(&format!("unexpected EOF after {:?}", token), &args.input, line_count),
                }
            },

            Token::InstOp => {
                inst_counter += 1;
                lex.next();
                //println!("token {:?} has var", token);
                match lex.next() {
                    Some(Token::Name(var)) => {
                        if !variables.contains(&var) {
                            //println!("token {:?}, var {}", token, var);
                            variables.push(var);
                        }
                    },
                    Some(Token::SysVar(_)) => (),
                    Some(unx) => error(&format!("unexpected token {:?} after {:?}", unx, token), &args.input, line_count),
                    None => error(&format!("unexpected EOF after {:?}", token), &args.input, line_count),
                }
            },

            Token::InstRadar => {
                inst_counter += 1;
                for _ in 0..5 {
                    lex.next();
                }
                //println!("token {:?} has var", token);
                match lex.next() {
                    Some(Token::Name(var)) => {
                        if !variables.contains(&var) {
                            //println!("token {:?}, var {}", token, var);
                            variables.push(var);
                        }
                    },
                    Some(Token::SysVar(_)) => (),
                    Some(unx) => error(&format!("unexpected token {:?} after {:?}", unx, token), &args.input, line_count),
                    None => error(&format!("unexpected EOF after {:?}", token), &args.input, line_count),
                }
            },

            Token::InstUnitControl => {
                inst_counter += 1;
                match lex.next() {
                    Some(Token::SubInstGetBlock) => {
                        lex.next();
                        lex.next();
                        lex.next();
                        //println!("token {:?} has var", token);
                        for _ in 0..2 {
                            match lex.next() {
                                Some(Token::Name(var)) => {
                                    if !variables.contains(&var) {
                                        //println!("token {:?}, var {}", token, var);
                                        variables.push(var);
                                    }
                                },
                                Some(Token::SysVar(_)) => (),
                                Some(unx) => error(&format!("unexpected token {:?} after {:?}", unx, token), &args.input, line_count),
                                None => error(&format!("unexpected EOF after {:?}", token), &args.input, line_count),
                            }
                        }
                    },
                    Some(Token::SubInstWithin) => {
                        lex.next();
                        lex.next();
                        //println!("token {:?} has var", token);
                        match lex.next() {
                            Some(Token::Name(var)) => {
                                if !variables.contains(&var) {
                                    //println!("token {:?}, var {}", token, var);
                                    variables.push(var);
                                }
                            },
                            Some(Token::SysVar(_)) => (),
                            Some(unx) => error(&format!("unexpected token {:?} after {:?}", unx, token), &args.input, line_count),
                            None => error(&format!("unexpected EOF after {:?}", token), &args.input, line_count),
                        }
                    },
                    _ => (),
                }
            }

            Token::InstUnitLocate => {
                inst_counter += 1;
                match lex.next() {
                    Some(Token::SubInstOre) => {
                        lex.next();
                        //println!("token {:?} has var", token);
                        for _ in 0..3 {
                            match lex.next() {
                                Some(Token::Name(var)) => {
                                    if !variables.contains(&var) {
                                        //println!("token {:?}, var {}", token, var);
                                        variables.push(var);
                                    }
                                },
                                Some(Token::SysVar(_)) => (),
                                Some(unx) => error(&format!("unexpected token {:?} after {:?}", unx, token), &args.input, line_count),
                                None => error(&format!("unexpected EOF after {:?}", token), &args.input, line_count),
                            }
                        }
                    },
                    Some(Token::SubInstBuilding) => {
                        lex.next();
                        lex.next();
                        //println!("token {:?} has var", token);
                        for _ in 0..4 {
                            match lex.next() {
                                Some(Token::Name(var)) => {
                                    if !variables.contains(&var) {
                                        //println!("token {:?}, var {}", token, var);
                                        variables.push(var);
                                    }
                                },
                                Some(Token::SysVar(_)) => (),
                                _ => error(&format!("unexpected token after {:?}", token), &args.input, line_count),
                            }
                        }
                    },
                    Some(Token::SubInstSpawn) => {
                        //println!("token {:?} has var", token);
                        for _ in 0..3 {
                            match lex.next() {
                                Some(Token::Name(var)) => {
                                    if !variables.contains(&var) {
                                        //println!("token {:?}, var {}", token, var);
                                        variables.push(var);
                                    }
                                },
                                Some(Token::SysVar(_)) => (),
                                _ => error(&format!("unexpected token after {:?}", token), &args.input, line_count),
                            }
                        }
                    },
                    _ => (),
                }
            }

            Token::Label(name) => { labels.insert(name, inst_counter); },
            Token::Subroutine(name) => { routines.insert(name, inst_counter); },
            Token::Newline => { skip_to_next = false; line_count += 1; },

            Token::InstEndRoutine => (),

            _ => error(&format!("unexpected token {:?}", token), &args.input, line_count),
        }
        if skip_to_next {
            loop {
                let next = lex.next();
                //println!("pass 1 skipping {:?}", next);
                match next {
                    Some(Token::Newline) | None => break,
                    _ => ()
                }
            }
            line_count += 1;
        }
    }

    //println!("labels: {:?}", labels);
    //println!("variables: {:?}", variables);

    let instructions = create_instructions();

    // 2nd pass: fill out code
    lex = Token::lexer(&string); // do 'gain
    let mut code: Vec<String> = Vec::new();
    let mut current_subroutine: Option<String> = None;
    line_count = 1;
    let mut label_debug_name: Option<String> = None;
    while let Some(token) = lex.next() {
        //println!("pass 2 token {:?}", token);
        match token {
            Token::Label(name) => label_debug_name = Some(format!("label {}", name)), // we dont want these in the finished code
            Token::Subroutine(sub) => { label_debug_name = Some(format!("subroutine {}", sub)); current_subroutine = Some(sub) },
            Token::InstReturn => {
                if let Some(label) = &current_subroutine {
                    if args.debugcomments {
                        code.push(format!("set @counter {}Return # return from {}", label, label));
                    } else {
                        code.push(format!("set @counter {}Return", label));
                    }
                } else {
                    error("can't return outside of a subroutine!", &args.input, line_count);
                    std::process::exit(1);
                }
            },
            Token::InstEndRoutine => current_subroutine = None,
            Token::Newline => line_count += 1,
            _ => {
                let mut found_inst = false;
                for inst in instructions.iter() {
                    if token == inst.get_token() {
                        code.append(&mut inst.compile(&mut lex, &variables, &labels, &routines, &args.input, line_count, args.debugcomments, &current_subroutine, code.len()));
                        if let Some(name) = &label_debug_name {
                            if args.debugcomments {
                                let string = code.pop().unwrap();
                                code.push(format!("{} # {}", string, name));
                            }
                            
                            label_debug_name = None;
                        }
                        found_inst = true;
                        break;
                    }
                }
                if !found_inst {
                    error(&format!("unexpected token {}", lex.slice()), &args.input, line_count);
                    std::process::exit(1);
                }
            },
        }
    }

    code.push("".to_string()); // add another newline for good measure

    let code_string = code.join("\n");

    if args.output == "stdout" {
        println!("{}", code_string);
    } else {
        let mut file = File::create(&args.output).expect("unable to create file");
        file.write_all(code_string.as_bytes()).expect("unable to write");
        println!("Saved to {}", &args.output);
    }
}
