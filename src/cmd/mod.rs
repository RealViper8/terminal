use std::{thread, time::Duration};
use std::process::Command;
use std::io::stdout;

use crossterm::{
    execute,
    style::{Color, SetBackgroundColor, SetForegroundColor}
};
use crossterm::style::Color::Reset;
pub mod arguments;

pub struct Terminal {
    pub title: String
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Type {
    LPARREN,
    RPARREN,
    RBRACKET,
    LBRACKET,
    NONE,
    POINT,
    OPERATOR(String),
    NUMBER(f64),
    STRING(String),
}

pub fn prompt_credits(name: Option<bool>, font: Option<&String>) {
    execute!(stdout(), SetForegroundColor(Color::Cyan)).unwrap();
    if name != None && name == Some(true) {
        if !font.is_none() && font == Some(&"bold".to_string()) {
            println!(r#"--- 𝐂𝐨𝐦𝐦𝐚𝐧𝐝 𝐏𝐫𝐨𝐦𝐩𝐭 ---"#);
            println!(r#"  𝐌𝐚𝐝𝐞 𝐛𝐲 𝐫𝐞𝐚𝐥𝐯𝐢𝐩𝐞𝐫𝟖{}"#, "\n");
        } else {
            println!("--- Command Prompt ---");
            println!("  Made by realviper8\n");
        }
    } else {
        if !font.is_none() && font == Some(&"bold".to_string()) {
            println!(r#"--- 𝐂𝐨𝐦𝐦𝐚𝐧𝐝 𝐏𝐫𝐨𝐦𝐩𝐭 ---{}"#,"\n");
        } else {
            println!("--- Command Prompt ---\n");
        }
    }
    execute!(stdout(), SetForegroundColor(Reset)).unwrap();
}

pub trait Func {
    fn new(title: String) -> Self;
    fn change_color(color: i8);
    fn change_bg(color: i8);
}

impl Func for Terminal {
    fn new(title: String) -> Self {
        return Terminal {
            title,
        }
    }
    
    fn change_color(color: i8) {
        if color == 0 {
            execute!(stdout(), SetForegroundColor(Color::Green)).unwrap();
        } else if color == 1 {
            execute!(stdout(), SetForegroundColor(Color::Blue)).unwrap();
        } else if color == 2 {
            execute!(stdout(), SetForegroundColor(Color::Black)).unwrap();
        } else if color == 3 {
            execute!(stdout(), SetForegroundColor(Color::Grey)).unwrap();
        } else if color == 4 {
            execute!(stdout(), SetForegroundColor(Color::Red)).unwrap();
        } else if color == 5 {
            execute!(stdout(), SetForegroundColor(Color::White)).unwrap();
        } else if color == 6 {
            execute!(stdout(), SetForegroundColor(Color::Magenta)).unwrap();
        }  else if color == 7 {
            execute!(stdout(), SetForegroundColor(Color::DarkGreen)).unwrap();
        }  else if color == 8 {
            execute!(stdout(), SetForegroundColor(Color::DarkBlue)).unwrap();
        }  else if color == 9 {
            execute!(stdout(), SetForegroundColor(Color::Yellow)).unwrap();
        } else if color == 10 {
            execute!(stdout(), SetForegroundColor(Color::Cyan)).unwrap();
        } else if color == 99 {
            execute!(stdout(), SetBackgroundColor(Reset)).unwrap();
        }
    }

    fn change_bg(color: i8) {
        if color == 0 {
            execute!(stdout(), SetBackgroundColor(Color::Green)).unwrap();
        } else if color == 1 {
            execute!(stdout(), SetBackgroundColor(Color::Blue)).unwrap();
        } else if color == 2 {
            execute!(stdout(), SetBackgroundColor(Color::Black)).unwrap();
        } else if color == 3 {
            execute!(stdout(), SetBackgroundColor(Color::Grey)).unwrap();
        } else if color == 4 {
            execute!(stdout(), SetBackgroundColor(Color::Red)).unwrap();
        } else if color == 5 {
            execute!(stdout(), SetBackgroundColor(Color::White)).unwrap();
        }
    }
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

#[cfg(target_os = "windows")]
pub fn clear() {
    Command::new("cmd").args(["/C","cls"]).spawn().unwrap();
    thread::sleep(Duration::new(0,500));
}

#[cfg(target_os = "macos")]
pub fn clear() {
    Command::new("sh").args(["-c","clear"]).spawn().unwrap();
    thread::sleep(Duration::new(0, 600));
}

#[cfg(target_os = "linux")]
pub fn clear() {
    Command::new("bash").args(["-c","clear"]).spawn().unwrap();
    thread::sleep(Duration::new(0, 550));
}