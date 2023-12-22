use std::collections::HashSet;

#[derive(std::fmt::Debug, PartialEq, Clone)]
pub enum Token {
    Dot,
    EndToken,
    Keyword(String),
    Class,
    Id(String),
    OpenBrace,
    CloseBrace,
    OpenSqBrace,
    CloseSqBrace,
    ColonEq,
    Def,
    Colon,
    Arrow,
    Int(usize),
    Operator(String),
    Str(String),
}

impl Token {
    pub fn as_dot(&self) -> Option<()> {
        match self {
            Token::Dot => Some(()),
            _ => None,
        }
    }
    pub fn as_arrow(&self) -> Option<()> {
        match self {
            Token::Arrow => Some(()),
            _ => None,
        }
    }
    pub fn as_end_token(&self) -> Option<()> {
        match self {
            Token::EndToken => Some(()),
            _ => None,
        }
    }
    pub fn as_keyword(&self) -> Option<String> {
        match self {
            Token::Keyword(name) => Some(name.to_owned()),
            _ => None,
        }
    }
    pub fn as_class(&self) -> Option<()> {
        match self {
            Token::Class => Some(()),
            _ => None,
        }
    }
    pub fn as_def(&self) -> Option<()> {
        match self {
            Token::Def => Some(()),
            _ => None,
        }
    }
    pub fn as_id(&self) -> Option<String> {
        match self {
            Token::Id(name) => Some(name.to_owned()),
            _ => None,
        }
    }
    pub fn as_open_sq_brace(&self) -> Option<()> {
        match self {
            Token::OpenSqBrace => Some(()),
            _ => None,
        }
    }
    pub fn as_close_sq_brace(&self) -> Option<()> {
        match self {
            Token::CloseSqBrace => Some(()),
            _ => None,
        }
    }
    pub fn as_open_brace(&self) -> Option<()> {
        match self {
            Token::OpenBrace => Some(()),
            _ => None,
        }
    }
    pub fn as_close_brace(&self) -> Option<()> {
        match self {
            Token::CloseBrace => Some(()),
            _ => None,
        }
    }
    pub fn as_colon_eq(&self) -> Option<()> {
        match self {
            Token::ColonEq => Some(()),
            _ => None,
        }
    }
    pub fn as_colon(&self) -> Option<()> {
        match self {
            Token::Colon => Some(()),
            _ => None,
        }
    }
    pub fn as_operator(&self) -> Option<String> {
        match self {
            Token::Operator(kind) => Some(kind.to_owned()),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<usize> {
        match self {
            Token::Int(val) => Some(*val),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<String> {
        match self {
            Token::Str(str) => Some(str.to_owned()),
            _ => None,
        }
    }
}

pub fn tokenize(program_string: String) -> Vec<Token> {
    let mut idx = 0;

    let mut tokens: Vec<Token> = vec![];

    let end_chars = HashSet::from([".", ";", " ", "}", ")"]);

    let one_char_operators = HashSet::from(["+", "-", "*", "/", "%", ">", "<", "=", "|", "&"]);
    let two_char_operators = HashSet::from(["**", ">=", "<=", "==", "&&", "||"]);

    while idx < program_string.len() {
        if program_string.get(idx..(idx + 2)) == Some("--") {
            idx += 2;
            while program_string.get(idx..(idx + 1)) != Some("\n") {
                idx += 1;
            }
        } else if program_string.get(idx..(idx + 1)) == Some("\n") {
            idx += 1;
        } else if program_string.get(idx..(idx + 1)) == Some(" ") {
            idx += 1;
        } else if program_string.get(idx..(idx + 2)) == Some(": ") {
            idx += 2;
            tokens.push(Token::Colon)
        } else if program_string.get(idx..(idx + 2)) == Some(":=") {
            idx += 2;
            tokens.push(Token::ColonEq)
        } else if program_string.get(idx..(idx + 2)) == Some("->") {
            idx += 2;
            tokens.push(Token::Arrow)
        } else if program_string.get(idx..(idx + 3)) == Some("end") {
            idx += 3;
            tokens.push(Token::EndToken)
        } else if let Some(op) = program_string
            .get(idx..(idx + 1))
            .filter(|item| one_char_operators.get(item).is_some())
        {
            idx += 1;
            tokens.push(Token::Operator(op.to_string()))
        } else if let Some(op) = program_string
            .get(idx..(idx + 2))
            .filter(|item| two_char_operators.get(item).is_some())
        {
            idx += 2;
            tokens.push(Token::Operator(op.to_string()))
        } else if program_string.get(idx..(idx + 1)) == Some(":") {
            idx += 1;
            let mut name = String::from("");
            while program_string
                .get(idx..(idx + 1))
                .and_then(|char| end_chars.get(char))
                .is_none()
            {
                if let Some(s) = program_string.get(idx..(idx + 1)) {
                    name += s;
                }
                idx += 1;
            }
            tokens.push(Token::Keyword(name));
        } else if program_string.get(idx..(idx + 1)) == Some(".") {
            idx += 1;
            tokens.push(Token::Dot);
        } else if program_string.get(idx..(idx + 1)) == Some(";") {
            idx += 1;

            tokens.push(Token::EndToken);
        } else if program_string.get(idx..(idx + 1)) == Some("{") {
            idx += 1;
            tokens.push(Token::OpenBrace);
        } else if program_string.get(idx..(idx + 1)) == Some("}") {
            idx += 1;
            tokens.push(Token::CloseBrace);
        } else if program_string.get(idx..(idx + 1)) == Some("[") {
            idx += 1;
            tokens.push(Token::OpenSqBrace);
        } else if program_string.get(idx..(idx + 1)) == Some("]") {
            idx += 1;
            tokens.push(Token::CloseSqBrace);
        } else if program_string.get(idx..(idx + 5)) == Some("class") {
            idx += 5;
            tokens.push(Token::Class);
        } else if program_string.get(idx..(idx + 3)) == Some("def") {
            idx += 3;
            tokens.push(Token::Def);
        } else if program_string.get(idx..(idx + 1)) == Some("\"") {
            idx += 1;
            let mut str = "".to_string();
            while let Some(val) = program_string.get(idx..(idx + 1)).filter(|x| x != &"\"") {
                str.push(val.chars().next().unwrap());
                idx += 1;
            }
            idx += 1;
            tokens.push(Token::Str(str));
        } else if program_string
            .chars()
            .skip(idx)
            .next()
            .map(|x| x.is_numeric())
            .unwrap_or(false)
        {
            let mut chars = program_string.chars().skip(idx);
            let mut val = chars.next().unwrap().to_string();
            let mut chars = chars.peekable();
            while chars.peek().is_some() && chars.peek().unwrap().is_numeric() {
                val.push(chars.next().unwrap());
            }
            idx += val.len();
            tokens.push(Token::Int(val.parse().unwrap()));
        } else if program_string
            .chars()
            .skip(idx)
            .next()
            .map(|x| x.is_alphabetic())
            .unwrap_or(false)
        {
            let mut chars = program_string.chars().skip(idx);
            let mut name = chars.next().unwrap().to_string();
            let mut chars = chars.peekable();
            while chars.peek().is_some() && chars.peek().unwrap().is_alphanumeric() {
                name.push(chars.next().unwrap());
            }
            idx += name.len();
            tokens.push(Token::Id(name));
        } else {
            println!("{:?}", tokens);
            panic!("no token matched")
        }
    }

    tokens
}
