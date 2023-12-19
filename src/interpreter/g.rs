#[derive(Debug)]
pub enum Tokens {
    OPERATOR(String),
    STRING(String),
    INTERGER(f64),
    PRINT,
    VAR,
    IF,
    ELSEIF,
    ELSE,
    NAME,
    RPARREN,
    LPARREN,
    EQUALS,
    EQUALSEQUALS,
    NOTEQUALS,
    GREATER,
    LESS,
}

pub fn tokenize(source: String) -> Result<Vec<Tokens>, String>  {
    let mut tokens: Vec<Tokens> = vec![];
    let mut curr = String::new();
    let mut str: i32 = 0;

    for i in source.chars() {
        match (i, &curr) {
            (_, cur) if cur == &String::from("print") => {
                tokens.push(Tokens::PRINT);
                curr = "".to_string();
            },
            (_, cur) if cur == &String::from("var") => {
                tokens.push(Tokens::VAR);
                curr = "".to_string();
            },
            ('(', _) => tokens.push(Tokens::LPARREN),
            (')', _) => tokens.push(Tokens::RPARREN),
            ('=', _) => tokens.push(Tokens::EQUALS),
            ('"', _) => {
                if str != 2 {
                    str += 1;
                    if str == 2 {
                        tokens.push(Tokens::STRING(curr.clone().trim().to_string()));
                        str = 0;
                    }
                }
            },
            _ => {
                if str != 2 {
                    curr.push(i)
                } else  if i == '(' {
                    tokens.push(Tokens::LPARREN)
                }
            },
        }

        println!()
    }
    println!("{}, {:?}", curr, tokens);
    Ok(vec![Tokens::OPERATOR("123".to_string())])
}