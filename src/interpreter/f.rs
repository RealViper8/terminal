use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub enum Tokens {
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Dot,
    Colon,
    Semi,
    Emit,
    CR,
    Drop,
    Dup,
    Swap,
    Over,
    Rot,
    And,
    OR,
    Invert,
    Dotquote,
    Quote,
    Less,
    Greater,
    EQ,
    IF,
    Else,
    Then,
    DO,
    Loop,
    Begin,
    Until,
    I,
    Variable,
    Constant,
    AT,
    Question,
    Bang,
    Bangitter,
    Allot,
    Cells,
    Key,
    String(String),
    Word(String),
    Int(i32),
}

const CELL_WIDTH: u8 = 1;

#[derive(Clone)]
pub struct Lexer {
    pub token_stack: VecDeque<Tokens>,
}

pub struct Loop {
    cur_iter: i32,
    in_loop: bool,
}

pub struct Interpreter {
    stack: Vec<i32>,
    if_stack: Vec<bool>,
    memory_stack: Vec<i32>,
    memory_map: std::collections::HashMap<String, usize>,
    constant_map: std::collections::HashMap<String, i32>,
    word_map: std::collections::HashMap<String, VecDeque<Tokens>>,
    in_word: bool,
    cur_loop: Loop,
    loop_tokens: VecDeque<Tokens>,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            token_stack: VecDeque::new(),
        }
    }

    pub fn get_token_from_word(&self, word: &str) -> Tokens {
        match word {
            "+" => Tokens::Plus,
            "-" => Tokens::Minus,
            "*" => Tokens::Mul,
            "/" => Tokens::Div,
            "mod" => Tokens::Mod,
            "." => Tokens::Dot,
            ":" => Tokens::Colon,
            ";" => Tokens::Semi,
            "emit" => Tokens::Emit,
            "cr" => Tokens::CR,
            "drop" => Tokens::Drop,
            "dup" => Tokens::Dup,
            "swap" => Tokens::Swap,
            "over" => Tokens::Over,
            "rot" => Tokens::Rot,
            "and" => Tokens::And,
            "or" => Tokens::OR,
            "invert" => Tokens::Invert,
            ".\"" => Tokens::Dotquote,
            "\"" => Tokens::Quote,
            "<" => Tokens::Less,
            ">" => Tokens::Greater,
            "=" => Tokens::EQ,
            "if" => Tokens::IF,
            "else" => Tokens::Else,
            "then" => Tokens::Then,
            "do" => Tokens::DO,
            "loop" => Tokens::Loop,
            "begin" => Tokens::Begin,
            "until" => Tokens::Until,
            "i" => Tokens::I,
            "variable" => Tokens::Variable,
            "constant" => Tokens::Constant,
            "@" => Tokens::AT,
            "?" => Tokens::Question,
            "!" => Tokens::Bang,
            "+!" => Tokens::Bangitter,
            "allot" => Tokens::Allot,
            "cells" => Tokens::Cells,
            "key" => Tokens::Key,
            _ => match word.parse::<i32>() {
                Ok(num) => Tokens::Int(num),
                Err(_) => Tokens::Word(word.to_string()),
            },
        }
    }

    pub fn generate_tokens(&mut self, filename: &str) {
        let raw_data = std::fs::read_to_string(filename).expect("Failed to read file");
        let data: VecDeque<&str> = raw_data.split_whitespace().collect();
        self.lex_data(data);
    }

    fn push(&mut self, value: Tokens) {
        self.token_stack.push_back(value);
    }

    fn lex_data(&mut self, mut data: VecDeque<&str>) {
        while !data.is_empty() {
            let mut word: String = Default::default();
            match data.pop_front() {
                Some(some_word) => {
                    word = some_word.to_string();
                }
                None => panic!("Unknown keyword {}", word),
            }
            let token = self.get_token_from_word(&word);
            match token {
                Tokens::Dotquote => {
                    let str: Vec<&str> = data
                        .clone()
                        .into_iter()
                        .take_while(|&v| self.get_token_from_word(v) != Tokens::Quote)
                        .collect();
                    data = data.split_off(str.len());
                    self.push(Tokens::String(str.join(" ")));
                }
                _ => self.push(token),
            }
        }
    }
}

macro_rules! binexpr {
    ($interpreter:ident, $symbol:tt) => {
        let a = $interpreter.pop();
        let b = $interpreter.pop();
        $interpreter.push((b $symbol a) as i32);
    }
}

macro_rules! compexpr {
    ($interpreter:ident, $symbol:tt) => {
        let a = $interpreter.pop();
        let b = $interpreter.pop();
        $interpreter.push(-((b $symbol a) as i32));
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            if_stack: Vec::new(),
            memory_stack: Vec::new(),
            memory_map: std::collections::HashMap::new(),
            constant_map: std::collections::HashMap::new(),
            word_map: std::collections::HashMap::new(),
            in_word: false,
            cur_loop: Loop {
                cur_iter: 0,
                in_loop: false,
            },
            loop_tokens: VecDeque::new(),
        }
    }

    fn push(&mut self, value: i32) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> i32 {
        match self.stack.pop() {
            Some(num) => num,
            None => panic!("empty stack"),
        }
    }

    fn dup(&mut self) {
        let a = self.pop();
        self.push(a);
        self.push(a);
    }

    fn swap(&mut self) {
        let a = self.pop();
        let b = self.pop();
        self.push(a);
        self.push(b);
    }

    fn over(&mut self) {
        let a = self.pop();
        let b = self.pop();
        self.push(b);
        self.push(a);
        self.push(b);
    }

    fn rot(&mut self) {
        let a = self.pop();
        let b = self.pop();
        let c = self.pop();
        self.push(b);
        self.push(a);
        self.push(c);
    }

    fn invert(&mut self) {
        let a = self.pop();
        self.push(-a - 1);
    }

    fn handle_word(&mut self, word: &String) {
        match self.word_map.get(word) {
            Some(tokens) => {
                self.in_word = true;
                self.interpret_tokens(tokens.clone());
            }
            None => match self.memory_map.get(word) {
                Some(index) => self.push(*index as i32),
                None => match self.constant_map.get(word) {
                    Some(value) => self.push(*value),
                    None => panic!("undefined word {}", word),
                },
            },
        }
    }

    fn add_word(&mut self, word: String, tokens: VecDeque<Tokens>) {
        self.word_map.insert(word, tokens);
    }

    pub fn interpret_tokens(&mut self, mut tokens: VecDeque<Tokens>) {
        while !tokens.is_empty() {
            if let Some(token) = tokens.pop_front() {
                match token {
                    Tokens::Plus => {
                        binexpr!(self, +);
                    }
                    Tokens::Minus => {
                        binexpr!(self, -);
                    }
                    Tokens::Mul => {
                        binexpr!(self, *);
                    }
                    Tokens::Div => {
                        binexpr!(self, /);
                    }
                    Tokens::Mod => {
                        binexpr!(self, %);
                    }
                    Tokens::Dot => print!("{}", self.pop()),
                    Tokens::Colon => {
                        let word_token = tokens.pop_front();
                        let current_word: String = match word_token {
                            Some(Tokens::Word(name)) => name.to_string(),
                            _ => panic!("expected word"),
                        };

                        let token_vec: VecDeque<Tokens> = tokens
                            .clone()
                            .into_iter()
                            .take_while(|v| *v != Tokens::Semi)
                            .collect();
                        tokens = tokens.split_off(token_vec.len());
                        self.add_word(current_word, token_vec);
                    }
                    Tokens::Semi => {
                        self.in_word = false;
                    }
                    Tokens::Emit => print!("{}", (self.pop() as u8) as char),
                    Tokens::CR => println!(),
                    Tokens::Drop => {
                        self.pop();
                    }
                    Tokens::Dup => self.dup(),
                    Tokens::Swap => self.swap(),
                    Tokens::Over => self.over(),
                    Tokens::Rot => self.rot(),
                    Tokens::And => {
                        binexpr!(self, &);
                    }
                    Tokens::OR => {
                        binexpr!(self, |);
                    }
                    Tokens::Invert => self.invert(),
                    Tokens::Dotquote => {
                        let str: Vec<Tokens> = tokens
                            .clone()
                            .into_iter()
                            .take_while(|v| *v != Tokens::Quote)
                            .collect();
                        println!("{:?}", str);
                    }
                    Tokens::Quote => (),
                    Tokens::Less => {
                        compexpr!(self, <);
                    }
                    Tokens::Greater => {
                        compexpr!(self, >);
                    }
                    Tokens::EQ => {
                        compexpr!(self, ==);
                    }
                    Tokens::IF => {
                        if !self.in_word {
                            panic!("expected to be in word");
                        }
                        let a = self.pop();
                        self.if_stack.push(a != 0);
                        if a == 0 {
                            let token_vec: VecDeque<Tokens> = tokens
                                .clone()
                                .into_iter()
                                .take_while(|v| (*v != Tokens::Then) && (*v != Tokens::Else))
                                .collect();
                            tokens = tokens.split_off(token_vec.len() + 1);
                        } else {
                            continue;
                        }
                    }
                    Tokens::Else => {
                        if !self.in_word {
                            panic!("expected to be in word");
                        }
                        let a = self.if_stack.pop().expect("Stack underflow");
                        if a {
                            let token_vec: VecDeque<Tokens> = tokens
                                .clone()
                                .into_iter()
                                .take_while(|v| *v != Tokens::Then)
                                .collect();
                            tokens = tokens.split_off(token_vec.len() + 1);
                        } else {
                            continue;
                        }
                    }
                    Tokens::Then => {}
                    Tokens::DO => {
                        let a = self.pop();
                        self.cur_loop.in_loop = true;
                        self.cur_loop.cur_iter = a;
                        let b = self.pop();
                        let token_vec: VecDeque<Tokens> = tokens
                            .clone()
                            .into_iter()
                            .take_while(|v| *v != Tokens::Loop)
                            .collect();
                        tokens = tokens.split_off(token_vec.len());
                        while self.cur_loop.cur_iter < b {
                            self.interpret_tokens(token_vec.clone());
                            self.cur_loop.cur_iter += 1;
                        }
                    }
                    Tokens::Loop => {
                        self.cur_loop.in_loop = false;
                    }
                    Tokens::Begin => {
                        self.loop_tokens = tokens
                            .clone()
                            .into_iter()
                            .take_while(|v| *v != Tokens::Until)
                            .collect();
                        self.loop_tokens.push_back(Tokens::Until);
                    }
                    Tokens::Until => {
                        if self.pop() == 0 {
                            self.interpret_tokens(self.loop_tokens.clone())
                        }
                    }
                    Tokens::I => {
                        if self.cur_loop.in_loop {
                            self.push(self.cur_loop.cur_iter);
                        } else {
                            panic!("error: i cannot be outside loop");
                        }
                    }
                    Tokens::Variable => match tokens.pop_front() {
                        Some(Tokens::Word(name)) => {
                            self.memory_map.insert(name, self.memory_stack.len());
                            self.memory_stack.push(0);
                        }
                        _ => panic!("unexpected keyword"),
                    },
                    Tokens::Constant => {
                        let a = tokens.pop_front().expect("Stack underflow");
                        let b = self.pop();
                        match a {
                            Tokens::Word(name) => self.constant_map.insert(name, b),
                            _ => panic!("error: unexpected keyword"),
                        };
                    }
                    Tokens::AT => {
                        let a = self.pop();
                        self.push(self.memory_stack[a as usize]);
                    }
                    Tokens::Question => {
                        let a = self.pop();
                        print!("{}", self.memory_stack[a as usize]);
                    }
                    Tokens::Bang => {
                        let a = self.pop();
                        let b = self.pop();
                        self.memory_stack[a as usize] = b;
                    }
                    Tokens::Bangitter => {
                        let a = self.pop();
                        self.memory_stack[a as usize] += 1;
                    }
                    Tokens::Allot => {
                        let a = self.pop();
                        self.memory_stack.append(&mut vec![0; a as usize]);
                    }
                    Tokens::Cells => {
                        let a = self.pop();
                        self.push(a * CELL_WIDTH as i32);
                    }
                    Tokens::Key => {
                        let mut input = String::new();
                        std::io::stdin()
                            .read_line(&mut input)
                            .expect("could not read line");
                        let key = input.as_bytes()[0] as i32;
                        self.push(key);
                    }
                    Tokens::String(str) => print!("{}", str),
                    Tokens::Word(word) => {
                        self.handle_word(&word);
                    }
                    Tokens::Int(num) => self.push(num),
                }
            };
        }
    }
}
