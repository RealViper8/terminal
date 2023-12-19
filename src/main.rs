use std::env::current_dir;
use std::{fs, io};
use cmd::*;
mod cmd;
mod interpreter;

use configparser::ini::Ini;
use std::io::prelude::Write;
use std::path::Path;
use std::process::{Command, exit};
use crate::interpreter::g::tokenize;

fn main() {
    let mut config = Ini::new();
    let mut color = 1;

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd").args(["/C","title","Terminal"]).spawn().unwrap();
    }

    if Path::new("config.ini").exists() {
        config.read(fs::read_to_string("config.ini").unwrap()).unwrap();

        color = if config.get("terminal","color") != None  {
            if let Err(_) = config.get("terminal","color").unwrap().parse::<i8>() {
                println!("\x1b[1;35mWarning: \x1b[0;31mIn config.ini in section terminal the value color should be a number between {}\n\x1b[0m", "0-6");
                1
            } else {
                config.get("terminal","color").unwrap().parse::<i8>().unwrap()
            }
        } else {
            1
        }

    } else {
        config.set("app","debug",Some("false".to_owned()));
        config.set("terminal","language",Some("english".to_owned()));
        config.set("terminal","color",Some("1".to_owned()));
        config.write("config.ini").unwrap();
    }

    let mut debug = if config.get("app","debug") != None {
        if let Err(_) = config.get("app","debug").unwrap().parse::<bool>() {
            println!("\x1b[1;31mError: \x1b[0;31mfailed to convert debug value to boolean\x1b[0m");
            Terminal::change_color(color);
            false
        } else {
            config.get("app","debug").unwrap().parse::<bool>().unwrap()
        }
    } else {
        false
    };

    prompt_credits();
    Terminal::change_color(color);

    let mut stdout = io::stdout();

    loop {
        let mut input: String = String::new();
        write!(stdout, "{}\x1b[0;32m$: ", current_dir().unwrap().display()).unwrap();
        stdout.flush().unwrap();
        match (io::stdin().read_line(&mut input), debug) {
            (Ok(n), true) => println!("Bytes read: {}\n", n),
            (Err(_) , false) => println!("\x1b[1;31mError: \x1b[0;31mfailed to readline\x1b[0m"),
            (Err(e) , true) => println!("\x1b[1;31mError: \x1b[0;31m{}\x1b[0m", e),
            _ => ()
        }

        let mut args = input.trim().split_whitespace();
        let args_count = args.clone().count();

        if args_count >= 2 {
            let first_arg = args.next().unwrap();
            let second_arg = args.next().unwrap();

            match (first_arg, second_arg) {
                ("color", "?") => {
                    println!("\x1b[1;36mColors:\x1b[0;32m");
                    Terminal::change_color(0);
                    println!("\t0\tIts Green");
                    Terminal::change_color(1);
                    println!("\t1\tIts Blue");
                    Terminal::change_color(2);
                    Terminal::change_bg(5);

                    print!("\t2\tIts Black");
                    Terminal::change_color(99);
                    println!();
                    Terminal::change_color(3);
                    println!("\t3\tIts Grey");
                    Terminal::change_color(4);
                    println!("\t4\tIts Red");
                    Terminal::change_color(5);
                    println!("\t5\tIts White");
                    Terminal::change_color(6);
                    println!("\t6\tIts Magenta");
                    println!();
                }
                ("debug", second_arg) if !second_arg.is_empty() => {
                    if second_arg.trim() == "on" || second_arg.trim() == "true" {
                        config.set("app","debug",Some("true".to_owned()));
                        config.write("config.ini").unwrap();
                        debug = true;

                        println!("\x1b[1;32mSuccesfully: \x1b[0;32mChanged debug to {}", second_arg);
                    } else if second_arg.trim() == "off" || second_arg.trim() == "false" {
                        config.set("app","debug",Some("false".to_owned()));
                        config.write("config.ini").unwrap();
                        debug = false;

                        println!("\x1b[1;32mSuccesfully: \x1b[0;32mdeactivatet debug")
                    }
                }
                ("color", second_arg) if !second_arg.is_empty() => {
                    if debug == true {
                        if let Err(e) = second_arg.parse::<i8>() {
                            println!("\x1b[1;31mError: \x1b[0;31m{}\x1b[0m\n", e);
                        }
                    } else {
                        if let Err(_) = second_arg.parse::<i8>() {
                            println!("\x1b[1;31mError: \x1b[0;31mSecond Argument should be a digit\x1b[0m");
                        }
                    }

                    if second_arg.parse::<i8>().unwrap() <= 10 {
                        color = second_arg.parse::<i8>().unwrap();
                        config.set("terminal","color",Some(second_arg.to_owned()));
                        config.write("config.ini").unwrap();
                    }
                }
                ("print", second_arg) if !second_arg.is_empty() => {
                    let mut tokens: Vec<Type> = vec![];
                    let mut count: usize = 0;
                    let mut string = String::new();
                    let mut str = 0;

                    for i in input.trim().replace(first_arg,"").chars() {
                        match (i, str) {
                            ('(', _) => {
                                tokens.insert(count, Type::LPARREN);
                            }

                            (char, 1) => {
                                if char != '"' {
                                    string.push(i);
                                } else {
                                    string.push(i);
                                    tokens.insert(count, Type::STRING(string.clone().replace('\"',"")));
                                    str += 1;
                                }
                            }
                            ('"', _) => {
                                if str == 2 {
                                    string.push(i);
                                    count += 1;
                                } else {
                                    string.push(i);
                                    str += 1;
                                }
                            }
                            (')', _) => {
                                tokens.insert(count, Type::RPARREN);
                                count += 1;
                            }
                            _ => ()
                        }
                    }

                    if debug == true {
                        println!("Tokens: {:?}\nString: {}",tokens, string.trim());
                    }

                    println!("\n{}",string);
                }
                ("g", second_arg) if second_arg.ends_with(".g") => {
                    if Path::new(second_arg.trim()).exists() {
                        let lex = tokenize(fs::read_to_string(second_arg).unwrap());
                    }
                }
                _ => ()
            }
        }

        match &*input.trim() {
            "help" => {
                println!("\x1b[1;36mhelp:");
                println!("\t\x1b[1;32mhelp\t\x1b[0;32mShows this menu");
                println!("\t\x1b[1;32mclear\t\x1b[0;32mClears the terminal");
                println!("\t\x1b[1;32mdebug\t\x1b[0;32mTurn debug on or off");
                println!("\t\x1b[1;32mexit\t\x1b[0;32mExit out of the terminal");
                println!("\t\x1b[1;32mcolor\t\x1b[0;32mChange the color of the terminal");
                println!();
            }
            "cls" | "clear" => clear(),
            "credits" | "title" => prompt_credits(),
            "exit" => exit(0),
            "debug" => {
                println!("\x1b[0;35mE.g:\n\t\x1b[0;32mdebug on\n\t\x1b[0;31mdebug off");
            }
            "echo" => {
                println!("\x1b[1;36mecho:\n");
                println!("\t\x1b[0;32mprints message on screen\n\tExample: echo \"Hello, World!\"");
            }
            "color" => println!("\x1b[0;32mType color ? to view a list of colors\n\x1b[0;35mE.g color 1\n"),
            "g" => {
                println!("\x1b[1;36mg:\n");
                println!("\t\x1b[0;32mIs a interpreter that I made usage is g <filename.g>");
            }
            _ => {
                if input.trim().contains("print") {
                    let mut tokens: Vec<Type> = vec![];
                    let mut count: usize = 0;
                    let mut string = String::new();
                    let mut str = 0;

                    for i in input.trim().chars() {
                        match (i, str) {
                            ('(', _) => {
                                tokens.insert(count, Type::LPARREN);
                            }
                            (char, 1) => {
                                if char != '"' {
                                    string.push(i);
                                } else {
                                    string.push(i);
                                    tokens.insert(count, Type::STRING(string.clone().replace('\"',"")));
                                    str += 1;
                                }
                            }
                            ('"', _) => {
                                if str == 2 {
                                    string.push(i);
                                    count += 1;
                                } else {
                                    string.push(i);
                                    str += 1;
                                }
                            }
                            (')', _) => {
                                tokens.insert(count, Type::RPARREN);
                                count += 1;
                            }
                            _ => ()
                        }
                    }

                    if debug == true {
                        println!("Tokens: {:?}\nString: {}",tokens, string.trim());
                    }

                    println!("\n{}",string);
                } else if input.trim().contains("(") && input.trim().contains(")") {
                    let mut tokens: Vec<Type> = vec![];

                    for i in input.trim().chars() {
                        match i {
                            '(' => tokens.push(Type::LPARREN),
                            ')' => tokens.push(Type::RPARREN),
                            '[' => tokens.push(Type::LBRACKET),
                            ']' => tokens.push(Type::RBRACKET),
                            '.' | ',' => tokens.push(Type::POINT),
                            ' ' => tokens.push(Type::NONE),
                            '+' | '-' | '/' | '*' => tokens.push(Type::OPERATOR(i.to_string())),
                            alpha if alpha.is_numeric() => tokens.push(Type::NUMBER(i.to_string().parse::<f64>().unwrap())),
                            _ => ()
                        }
                    }

                    if debug == true {
                        println!("Tokens: {:?}\n",tokens);
                    }

                    let mut final_str: String = String::new();
                    let mut previous: Type = Type::NONE;

                    for i in tokens {
                        match i {
                            Type::RPARREN => {
                                final_str.push(')');
                            },
                            Type::LPARREN => {
                                final_str.push('(');
                            },
                            Type::POINT => {
                                previous = Type::POINT;
                                final_str.push('.')
                            },
                            Type::OPERATOR(op) => {
                                if op == String::from("-") {
                                    if previous != Type::STRING(op.clone()) {
                                        previous = Type::OPERATOR(op.clone());
                                    }
                                    final_str.push('-')
                                } else if op == String::from("+") {
                                    if previous != Type::STRING(op.clone()) {
                                        previous = Type::OPERATOR(op.clone());
                                    }
                                    final_str.push('+')
                                } else if op == String::from("/") {
                                    if previous != Type::STRING(op.clone()) {
                                        previous = Type::OPERATOR(op.clone());
                                    }
                                    final_str.push('/')
                                } else if op == String::from("*") {
                                    if previous != Type::STRING(op.clone()) {
                                        previous = Type::OPERATOR(op.clone());
                                    }
                                    final_str.push('*')
                                }
                            },
                            Type::NUMBER(num) => {
                                if previous == Type::NONE && previous != Type::NUMBER(num) {
                                    previous = Type::NUMBER(num);
                                    final_str.push(num.to_string().parse::<char>().unwrap());
                                    numbers += 1
                                } else {
                                    previous = Type::NUMBER(num);
                                    final_str.push(num.to_string().parse::<char>().unwrap());
                                    numbers += 1
                                }
                            },

                            _ => {
                                numbers = 0
                            },
                        }
                    }


                    println!("\n{}",final_str);
                }
            }
        }
        Terminal::change_color(color);
    }
}