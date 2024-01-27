use std::env::{current_dir, set_current_dir, args};

use std::ops::Index;
use std::{fs, io};
use cmd::*;
mod cmd;
mod interpreter;

use configparser::ini::Ini;
use interpreter::math::MathInterpreter;
use std::io::prelude::Write;
use std::path::Path;
use std::process::Command;
use std::process::exit;
use crate::interpreter::f;


fn help(debug: bool, font: Option<&String>) {
    if debug == true {
        println!("Font: {:?}", font);
    }
    println!("\x1b[1;36mhelp:");
    println!("\t\x1b[1;32mhelp\t\x1b[0;32mShows this menu");
    println!("\t\x1b[1;32mclear\t\x1b[0;32mClears the terminal");
    println!("\t\x1b[1;32mdebug\t\x1b[0;32mTurn debug on or off");
    println!("\t\x1b[1;32mexit\t\x1b[0;32mExit out of the terminal");
    println!("\t\x1b[1;32mcolor\t\x1b[0;32mChange the color of the terminal");
    println!("\t\x1b[1;32mdir\t\x1b[0;32mLists all directories in cd");
    println!("\t\x1b[1;32mcd\t\x1b[0;32mChange the current working directory");
    println!("\t\x1b[1;32mprint (\"test\")\t\x1b[0;32mPrints text in this e.g test");
    println!("\t\x1b[1;32mf \t\x1b[0;32mForth Interpreter");
    println!("\t\x1b[1;32mstart\t\x1b[0;32mStart a command usage start cmd or for unix start ls");
    println!("\t\x1b[1;32mpwd\t\x1b[0;32mShow the current path");
    println!("\t\x1b[1;32mprompt\t\x1b[0;32mChange the prompt message to something else");
    println!("\t\x1b[1;32meditcf\t\x1b[0;32mEdits a config option from config.ini");
    println!("\t\x1b[1;32mviewcf\t\x1b[0;32mLists all the configs of config.ini");
    println!("\t\x1b[1;32mcredits\t\x1b[0;32mShows credits");
    println!("\t\x1b[1;32mcheck\t\x1b[0;32mChecks if file or directory exists");
    println!("\t\x1b[1;32mmath\t\x1b[0;32mUse math interpreter e.g math 2+2");
    println!("\t\x1b[1;32mviewfile config.ini\t\x1b[0;32mPrints the contents of a file in this e.g the contents of the config.ini");
    println!("\t\x1b[1;32mignore-warnings\t\x1b[0;32mIgnores warnings at startup");
    println!();
}

fn main() {
    let mut warnings = 0;
    let mut warnings_text: Vec<String> = vec![];
    let mut check_args: Vec<String> = args().collect();
    check_args.remove(0);


    let mut config = Ini::new();
    let mut color = 1;
    let commands = vec!["cd", "help", "color", "print", "editcf", "edit", "check", "check_path", "math", "viewfile"];
    let config_dir = current_dir().unwrap().display().to_string();
    let mut ls = String::from("default");

    // FONT is now experimental please don't use it
    let mut font = String::from("default");
    let mut clear_type = String::from("default");
    let mut ignore_warnings = false;

    let home_directory: &str = if cfg!(target_os = "linux") {
        "/bin"
    } else if cfg!(target_os = "windows") {
        "C:/Windows"
    } else if cfg!(target_os = "macos") {
        "~/Applications"
    } else {
        "/"
    };

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd").args(["/C","title","terminal"]).spawn().unwrap();
    }

    if Path::new("config.ini").exists() {
        config.read(fs::read_to_string("config.ini").unwrap()).unwrap();
        
        color = if config.get("terminal","color") != None  {
            if let Err(_) = config.get("terminal","color").unwrap().parse::<i8>() {
                warnings_text.push(format!("\x1b[1;33mWarning: \x1b[0;33mcolor value should be a number between {} and {} in config.ini\nUsing default wich is {}\x1b[0m", 0, 10, "1/blue\n").to_string());
                warnings += 1;
                1
            } else {
                config.get("terminal","color").unwrap().parse::<i8>().unwrap()
            }
        } else {
            1
        };

        ignore_warnings = if config.get("terminal","ignore-warnings") != None  {
            if let Err(_) = config.get("terminal","ignore-warnings").unwrap().parse::<bool>() {
                false
            } else {
                if config.get("terminal","ignore-warnings").unwrap() == "true" {
                    true
                } else {
                    false
                }
            }
        } else {
            false
        };

        ls = if config.get("terminal","ls") != None {
            if config.get("terminal","ls").unwrap() == "list" {
                String::from("list")
            } else if config.get("terminal", "ls").unwrap() == "list2" {
                String::from("list2")
            } else {
                String::from("default")
            }
        } else {
            "default".to_string()
        };

        font = if config.get("terminal", "font") != None {
            if config.get("terminal", "font").unwrap() == "bold" {
                "bold".to_string()
            } else {
                "default".to_string()
            }
        } else {
            String::from("default")
        };

        clear_type = if config.get("terminal","clear") != None {
            if config.get("terminal","clear").unwrap().to_uppercase() == "ANSI" {
                String::from("ansi").to_uppercase()
            } else {
                String::from("default")
            }
        } else {
            String::from("default")
        }

    } else {
        config.set("app","debug",Some("false".to_owned()));
        config.set("terminal","color",Some("1".to_owned()));
        config.set("terminal","prompt",Some("default".to_owned()));
        config.set("terminal","ls",Some("default".to_owned()));
        config.set("terminal","clear",Some("ansi".to_owned()));
        config.write("config.ini").unwrap();
    }

    let mut debug = if config.get("app","debug") != None {
        if let Err(_) = config.get("app","debug").unwrap().parse::<bool>() {
            warnings_text.push("Warning: debug value should be true or false in config.ini\nUsing default value: false".to_string());
            warnings += 1;
            Terminal::change_color(color);
            false
        } else {
            config.get("app","debug").unwrap().parse::<bool>().unwrap()
        }
    } else {
        false
    };

    if ignore_warnings == false && warnings >= 1 {
        let mut warn = 1;
        for warning in warnings_text {
            println!("\x1b[1;33m{}: \x1b[0;33m{}\x1b[0m", warn, warning);
            warn += 1;
        }
        println!("\n\x1b[1;33mWarnings: {}\x1b[0m", warnings);
        println!("\x1b[0;36mType ignore-warnings to ignore warnings !\n\x1b[0m");
    }

    if check_args.len() >= 1 {
        let mut final_args: Vec<String> = vec![];
        if debug == true {
            println!("List of Args: {:?}", check_args);
        }
        match arguments::parse(check_args) {
            Ok(args) => {
                for arg in args {
                    final_args.push(arg);
                }
            },
            Err(e) => println!("Error: {}", e),
        }
        let arg_1 = final_args.index(0);

        if arg_1 == "help" {
            help(debug, Some(&font));
        } else if arg_1 == "cmd" {
            if cfg!(target_os = "windows") {
                Command::new("cmd").args(["start","cmd"]).status().unwrap();
            } else if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
                Command::new("bash").status().unwrap();
            }
        }
        print!("\x1b[0m");
        exit(0)
    }
    
    prompt_credits(None, Some(&font));
    Terminal::change_color(color);

    loop {
        let current = &*current_dir().unwrap().display().to_string();
        Terminal::change_color(color);
        let mut input: String = String::new();

        if Path::new("config.ini").exists() {
            config.read(fs::read_to_string("config.ini").unwrap()).unwrap();
            if config.get("terminal","prompt").is_none() {
                print!("{}\x1b[1;32m$: ", current_dir().unwrap().display());
            } else {
                if config.get("terminal","prompt").unwrap() == "default" {
                    config.remove_key("terminal", "prompt");
                    print!("{}\x1b[0;32m$: ", current_dir().unwrap().display());
                } else {
                    print!("{}\x1b[0;32m$: ", config.get("terminal","prompt").unwrap());
                }
            }
        } else {
            let current_d = current_dir().unwrap().display().to_string();
            set_current_dir(config_dir.clone()).unwrap();
            if let Err(e) = fs::read_to_string("config.ini") {
                if debug == true {
                    println!("\x1b[1;31mError: \x1b[0;31m{}", e);
                    println!("This happens if you delete config.ini while running! Start again the terminal to fix it\x1b[0m")
                } else {
                    println!("\x1b[1;31mError: \x1b[0;31mfailed to read config file\nThis happens if you delete config.ini while running! Start again the terminal to fix it\x1b[0m");
                }
                exit(1)
            }
            config.read(fs::read_to_string("config.ini").unwrap()).unwrap();
            if config.get("terminal","prompt").is_none() {
                print!("{}\x1b[0;32m$: ", current);
            } else {
                if config.get("terminal","prompt").unwrap() == "default" {
                    config.remove_key("terminal", "prompt");
                    print!("{}\x1b[0;32m$: ", current);
                } else {
                    print!("{}\x1b[0;32m$: ", config.get("terminal","prompt").unwrap());
                }
            }
            set_current_dir(current_d).unwrap();
        }

        io::stdout().flush().unwrap();
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
                ("math", arg) => {
                    let mut interpreter = MathInterpreter::new();
                    if let Err(e) = interpreter.evaluate(arg) {
                        if debug == true {
                            println!("\x1b[1;31mError: \x1b[0;31m{}\x1b[0m\n", e);
                        } else {
                            println!("\x1b[1;31mError: \x1b[0;31mFailed to evaluate math expression: \x1b[0;31m{}\x1b[0m\n", e);
                        }
                    } else {
                        println!("{}\n", interpreter.evaluate(arg).unwrap());
                    }
                }
                ("cd", arg) => {
                    if Path::new(arg).exists() {
                        if let Err(e) = set_current_dir(arg) {
                            if debug == true {
                                println!("\x1b[1;31mError: {}", e);
                            } else {
                                println!("\x1b[1;31mError: \x1b[0;31mFailed to change cd to {}:\x1b[0;36m No permission\x1b[0m", second_arg);
                            }
                        } else {
                            if debug == true {
                                println!("\x1b[1;36mCurrent Directory changed to \x1b[1;32m{}\x1b[0m", current_dir().unwrap().display());
                            }
                        }
                    } else {
                        if debug == true {
                            println!("\x1b[1;31mError: \x1b[0;31mFailed to change current directory: doesnt exist\x1b[0m");
                        }
                    }
                }
                ("viewfile", arg) if !arg.is_empty() => {
                    if(Path::new(arg)).exists() {
                        for i in fs::read_to_string(arg).unwrap().lines() {
                            println!("{}", i);
                        }
                    } else {
                        println!("\x1b[1;31m Error: \x1b[0;31mFile doesnt exist\x1b[0m");
                    }
                }
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
                    Terminal::change_color(7);
                    println!("\t7\tIts Dark Green");
                    Terminal::change_color(8);
                    println!("\t8\tIts Dark Blue");
                    Terminal::change_color(9);
                    println!("\t9\tIts Yellow");
                    Terminal::change_color(10);
                    println!("\t10\tIts Cyan");
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
                    continue;
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
                ("f", _) => {
                    if Path::new(second_arg.trim()).exists() {
                        let mut lexer = f::Lexer::new();
                        lexer.generate_tokens(second_arg);
                        let mut interpreter = f::Interpreter::new();
                        interpreter.interpret_tokens(lexer.token_stack);
                        print!("\n\n");
                    }
                }
                ("prompt", "--help" | "-help" | "?") => {        
                    println!("\x1b[1;36mprompt:");
                    println!("\t\x1b[0;32mPrompts a custom message instead of the current directory\n\tExample: prompt Hello, World!");
                    println!("\t\x1b[0;36mInfo: to restore do prompt default");
                }
                ("prompt", second_arg) => {
                    let mut config = Ini::new();
                    let current_d = current_dir().unwrap().display().to_string();

                    if Path::new("config.ini").exists() {
                        config.read(fs::read_to_string("config.ini").unwrap()).unwrap();
                        config.set("terminal","prompt", Some(second_arg.to_string()));
                    } else {
                        set_current_dir(current).unwrap();
                        config.set("terminal","prompt", Some(second_arg.to_string()));
                    }

                    config.write("config.ini").unwrap();
                    set_current_dir(current_d).unwrap();
                }
                ("start", "--help" | "-help" | "?") => {
                    println!("\x1b[1;36mstart:");
                    println!("\t\x1b[0;32mStarts a command\n\tExample: start help");
                }
                ("start", _) => {
                    if let Err(e) = Command::new(second_arg).spawn() {
                        println!("Failed to run command '{}' ", second_arg);
                        if debug == true {
                            println!("Error: {}", e);
                        } else {
                            println!("Command not found");
                        }
                    }
                }
                ("editcf" | "edit", second_arg) if second_arg.contains("=") => {
                    let value = second_arg.replace("prompt", "").replace("color", "").replace("debug", "").replace("=", "");
                    let option = second_arg.replace(&value, "");
                
                    set_current_dir(&config_dir).unwrap();
                    if option == "color=" {
                        let backup_color = config.get("terminal","color").unwrap().parse::<i8>().unwrap();

                        color = if let Ok(_) = value.parse::<i8>() {
                            value.parse().unwrap()
                        } else {
                            println!("\x1b[1;31mError: \x1b[0;31mMake sure you use a number for the option color from 1-10 !\x1b[0m");
                            backup_color
                        }
                    }
                    config.set("terminal", &option.replace("=", ""),Some(second_arg.replace(&option, "")));
                    config.write("config.ini").unwrap();
                    set_current_dir(current).unwrap();
                }
                ("check" | "check_path", second_arg) => {
                    if Path::new(second_arg.trim()).exists() {
                        println!("\x1b[1;32mPath: \x1b[0;32m'{}' exists\x1b[0m\n", second_arg);
                    } else {
                        println!("\x1b[1;31mPath: \x1b[0;31m'{}' does not exist\x1b[0m\n", second_arg);
                    }
                }
                (first_arg, _) if first_arg.contains("ch") || first_arg.contains("ck") => {
                    println!("Did you mean check_path?");
                }
                (first_arg, _) if first_arg.contains("ed") || first_arg.contains("it") => {
                    println!("Did you mean editcf?");
                }
                (first_arg, _) if first_arg.contains("pr") || first_arg.contains("om") => {
                    println!("Did you mean prompt?");
                }
                (first_arg, _) if first_arg.contains("st") || first_arg.contains("rt") => {
                    println!("Did you mean start?");
                }
                (first_arg, _) if first_arg.contains("cl") || first_arg.contains("ls") => {
                    println!("Did you mean clear?");
                }
                (first_arg, _) if first_arg.contains("de") || first_arg.contains("bg") => {
                    println!("Did you mean debug?");
                }
                (first_arg, _) if first_arg.contains("vi") || first_arg.contains("ew") => {
                    println!("Did you mean viewcf?");
                }
                _ => ()
            }
        }

        match &*input.trim() {
            "help" => {
                help(debug, Some(&font));
            }
            "math" => {
                println!("\nType exit or close to exit !\n");
                let mut input: String = String::new();
                let mut stdout = io::stdout();

                loop {
                    write!(stdout, "\x1b[0;32mCalculation: \x1b[0;36m").unwrap();
                    stdout.flush().unwrap();
                    io::stdin().read_line(&mut input).expect("failed to readline");
                    print!("\x1b[0m");
                    io::stdout().flush().unwrap();
                    if input.trim() != "exit" && input.trim() != "close" {
                        let mut interpreter = MathInterpreter::new();
                        if let Err(e) = interpreter.evaluate(&*input.trim()) {
                            if debug == true {
                                println!("\x1b[1;31mError: \x1b[0;31m{}\x1b[0m\n", e);
                            } else {
                                println!("\x1b[1;31mError: \x1b[0;31mFailed to evaluate math expression: \x1b[0;31m{}\x1b[0m\n", e);
                            }
                        } else {
                            println!("\x1b[1;32mResult: \x1b[0;32m{}\x1b[0m\n", interpreter.evaluate(&*input.trim()).unwrap());
                        }
                        input = "".to_string();
                    } else {
                        break;
                    }
                }
            }
            "ignore-warnings" | "ignore-warning" => {
                if ignore_warnings == false {
                    ignore_warnings = true;
                    println!("\x1b[1;32mInfo: \x1b[0;32mIgnoring warnings is set on\x1b[0m\n");
                } else {
                    ignore_warnings = false;
                    println!("\x1b[1;31mInfo: \x1b[0;31mIgnoring warnings is set off\x1b[0m\n");
                }
                set_current_dir(&config_dir).unwrap();
                config.set("terminal","ignore-warnings", Some(ignore_warnings.to_string()));
                config.write("config.ini").unwrap();
            }
            "pwd" => println!("\x1b[0m{}\n", current_dir().unwrap().display()),
            "cls" | "clear" => {
                match &*clear_type {
                    "default" => {
                        if debug == true {
                            println!("\x1b[0;36mClearing screen...");
                        }
                        clear();
                    }
                    "ANSI" => {
                        if debug == true {
                            println!("\x1b[0;36mClearing screen... with ANSI escape codes");
                        }
                        clear_screen();
                    },
                    _ => clear(),
                }
            },
            "credits" | "title" => prompt_credits(Some(true), Some(&font)),
            "exit" => {
                if debug == true {
                    println!("\x1b[0;36mExiting process with Code 0");
                }
                print!("\x1b[0m");
                exit(0);
            },
            first_arg if first_arg.contains("he") || first_arg.contains("lp") => {
                println!("Did you mean help?");
            }
            first_arg if first_arg.contains("ex") || first_arg.contains("it") => {
                println!("Did you mean exit?");
            }
            "viewcf" | "view" => {
                set_current_dir(&config_dir).unwrap();
                println!("{}", fs::read_to_string("config.ini").unwrap());
                set_current_dir(current).unwrap();
            }
            "editcf" | "edit" => {
                println!("\x1b[1;36meditcf:");
                println!("\t\x1b[0;32mEdits the config.ini\n\tExample: editcf color=9");
            }
            "debug" => {
                println!("\x1b[0;35mE.g:\n\t\x1b[0;32mdebug on\n\t\x1b[0;31mdebug off");
            }
            "color" => println!("\x1b[0;32mType color ? to view a list of colors\n\x1b[0;35mE.g color 1\n"),
            "f" | "forth" => {
                println!("\x1b[1;36mf:");
                println!("\t\x1b[0;32mIs a interpreter for forth usage is f <filename>");
            }
            "dir" | "ls" => {
                let mut dirs: Vec<String> = vec![];
                let mut files: Vec<String> = vec![];

                let paths = fs::read_dir("./").unwrap();

                for path in paths {
                    let path = path.unwrap().path().display().to_string().replace("./","");
                    if !path.contains(".") {
                        dirs.push(String::from(path))
                    } else {
                        files.push(String::from(path))
                    }
                }

                if debug == true {
                    println!("\x1b[1;32mDirectories: {}", dirs.iter().count());
                    println!("\x1b[0;36mFiles: {}\n", files.iter().count());
                }
                
                if ls == "default".to_string() {
                    println!("\x1b[1;32m{}", dirs.join(" "));
                    println!("\x1b[0;36m{}", files.join(" "));
                } else if ls == "list".to_string() {
                    println!("\x1b[1;32m{:?}", dirs);
                    println!("\x1b[0;36m{:?}", files);
                } else if ls == "list2".to_string() {
                    println!("\x1b[1;32m{}", dirs.join("\n"));
                    println!("\x1b[0;36m{}", files.join("\n"));
                }
                
                println!("\x1b[0m");
            }
            "cd" => println!("{}", current_dir().unwrap().display()),
            _ => {
                if input.trim().contains("print") {
                    let mut tokens: Vec<Type> = vec![];
                    let mut count: usize = 0;
                    let mut string = String::new();
                    let mut str = 0;

                    for i in input.trim().chars() {
                        match (i, str) {
                            ('(', _) => {
                                tokens.insert(count, Type::RPARREN);
                            }
                            (char, 1) => {
                                if char != '"' && char != '\'' {
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
                            ('\'', _) => {
                                if str == 2 {
                                    string.push(i);
                                    count += 1;
                                } else {
                                    string.push(i);
                                    str += 1;
                                }
                            }
                            (')', _) => {
                                tokens.insert(count, Type::LPARREN);
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
                    let mut numbers: i32 = 0;

                    if debug == true {
                        print!("Numbers: {}", numbers);
                    }

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
                } else if input.trim().starts_with("prompt") {
                    let text: String = input.trim().replace("prompt", "");

                    if Path::new("config.ini").exists() {
                        config.set("terminal", "prompt", Some(text));
                        config.write("config.ini").unwrap();
                    }
                } else {
                    let mut input_list = input.trim().split_whitespace();
                    let first_arg = input_list.next();

                    if debug == true {
                        println!("{:?}", first_arg);
                        println!("{:?}", input_list);
                    }

                    if first_arg == None {
                        if debug == true {
                            println!("Input is empty");
                        }
                    } else if commands.contains(&first_arg.unwrap()) {
                       continue;
                    } else {
                        if Path::new(home_directory).exists() {
                            set_current_dir(home_directory).unwrap();
                            if cfg!(target_os = "windows") {
                                if let Err(e) = Command::new(first_arg.unwrap()).current_dir(home_directory).status() {
                                    if debug == true {
                                        println!("\x1b[1;31mError: {}\x1b[0m\n", e);
                                    } else {
                                        println!("\x1b[1;31mError: \x1b[0;31mFailed to run `{}`: Doesnt exist\x1b[0m\n", first_arg.unwrap());
                                    }
                                } else {
                                    if debug == true {
                                        println!("\x1b[0;32m`{}` succesfully ran\x1b[0m\n", first_arg.unwrap());
                                    }
                                }
                            } else if cfg!(target_os = "linux") && Path::new(first_arg.unwrap()).exists() {
                                if let Err(e) = Command::new(first_arg.unwrap()).current_dir(home_directory).status() {
                                    if debug == true {
                                        println!("\x1b[1;31mError: {}\x1b[0m\n", e);
                                    } else {
                                        println!("\x1b[1;31mError: \x1b[0;31mFailed to run `{}`: Doesnt exist\x1b[0m\n", first_arg.unwrap());
                                    }
                                } else {
                                    if debug == true {
                                        println!("`{}` succesfully ran\n", first_arg.unwrap());
                                    }
                                }
                            } else if cfg!(target_os = "mcaos") && Path::new(first_arg.unwrap()).exists() {
                                if let Err(e) = Command::new(first_arg.unwrap()).current_dir(home_directory).status() {
                                    if debug == true {
                                        println!("\x1b[1;31mError: {}\x1b[0m\n", e);
                                    } else {
                                        println!("\x1b[1;31mError: \x1b[0;31mFailed to run `{}`: Doesnt exist\x1b[0m\n", first_arg.unwrap());
                                    }
                                } else {
                                    if debug == true {
                                        println!("`{}` succesfully ran\n", first_arg.unwrap());
                                    }
                                }
                            }
                            set_current_dir(current).unwrap();
                        } else {
                            println!("\x1b[1;31mError: \x1b[0;31mFailed to launch `{}`: Doesnt exist\n", first_arg.unwrap());
                            continue;
                        }
                    }
                }
            }
        }
        Terminal::change_color(color);
    }
}