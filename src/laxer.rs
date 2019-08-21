use std::str::Chars;
use std::iter::Peekable;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DfaState {
    Initial,
    Identifier,
    GreatThen,
    GreatEqual,
    IntLiteral,
    Int,
    IdInt1,
    IdInt2,
    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    Terminal,
}

#[derive(Debug)]
pub enum TokenType {
    Identifier,
    GreatThen,
    GreatEqual,
    IntLiteral,
    Int,
    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    Invalid,
    Eof,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    content: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", &self.token_type, &self.content)
    }
}

pub struct Laxer<'a> {
    char_iter: Peekable<Chars<'a>>,
    state: DfaState,
    content: String,
    verbose: bool,
}

impl From<DfaState> for TokenType {
    fn from(state: DfaState) -> Self {
        match state {
            DfaState::Initial => TokenType::Invalid,
            DfaState::Identifier => TokenType::Identifier,
            DfaState::GreatThen => TokenType::GreatThen,
            DfaState::GreatEqual => TokenType::GreatEqual,
            DfaState::IntLiteral => TokenType::IntLiteral,
            DfaState::Int => TokenType::Int,
            DfaState::IdInt1 => TokenType::Invalid,
            DfaState::IdInt2 => TokenType::Invalid,
            DfaState::Equal => TokenType::Equal,
            DfaState::Plus => TokenType::Plus,
            DfaState::Minus => TokenType::Minus,
            DfaState::Star => TokenType::Star,
            DfaState::Slash => TokenType::Slash,
            DfaState::Terminal => TokenType::Invalid,
        }
    }
}

impl Token {
    pub fn from(dfa_state: DfaState, content: String) -> Token {
        Token {
            token_type: dfa_state.into(),
            content,
        }
    }
}

impl<'a> Laxer<'a> {
    pub fn new(source: &'a str, verbose: bool) -> Laxer<'a> {
        Laxer {
            char_iter: source.chars().peekable(),
            state: DfaState::Initial,
            content: String::new(),
            verbose,
        }
    }

    fn next_state(&mut self) -> Result<Option<Token>, ()> {
        if let Some(ch) = self.char_iter.peek() {
            let current_state = self.state;
            self.state = match self.state {
                DfaState::Initial =>
                    match ch {
                        'i' => DfaState::IdInt1,
                        'a'..='z' | 'A'..='Z' | '_' | '$' => DfaState::Identifier,
                        '0'..='9' => DfaState::IntLiteral,
                        '>' => DfaState::GreatThen,
                        '=' => DfaState::Equal,
                        '+' => DfaState::Plus,
                        '-' => DfaState::Minus,
                        '*' => DfaState::Star,
                        '/' => DfaState::Slash,
                        _ => DfaState::Initial
                    },
                DfaState::Identifier =>
                    match ch {
                        '0'..='9' | 'a'..='z' | 'A'..='Z' | '_' | '$' => DfaState::Identifier,
                        _ => DfaState::Terminal
                    },
                DfaState::GreatThen =>
                    match ch {
                        '=' => DfaState::GreatEqual,
                        _ => DfaState::Terminal
                    },
                DfaState::GreatEqual => DfaState::Terminal,
                DfaState::IntLiteral =>
                    match ch {
                        '0'..='9' => DfaState::IntLiteral,
                        ' ' | '\n' | '\r' | '=' | '>' | '*' | '/' | '+' | '-' => DfaState::Terminal,
                        _ => {
                            self.content.push(self.char_iter.next().unwrap());
                            panic!("Invalid int literal {}", &self.content)
                        }
                    }
                ,
                DfaState::Int =>
                    match ch {
                        '0'..='9' | 'a'..='z' | 'A'..='Z' | '_' | '$' => DfaState::Identifier,
                        _ => DfaState::Terminal
                    },
                DfaState::IdInt1 =>
                    match ch {
                        'n' => DfaState::IdInt2,
                        '0'..='9' | 'a'..='z' | 'A'..='Z' | '_' | '$' => DfaState::Identifier,
                        _ => DfaState::Terminal,
                    },
                DfaState::IdInt2 =>
                    match ch {
                        't' => DfaState::Int,
                        '0'..='9' | 'a'..='z' | 'A'..='Z' | '_' | '$' => DfaState::Identifier,
                        _ => DfaState::Terminal
                    },
                DfaState::Equal => DfaState::Terminal,
                DfaState::Plus => DfaState::Terminal,
                DfaState::Minus => DfaState::Terminal,
                DfaState::Star => DfaState::Terminal,
                DfaState::Slash => DfaState::Terminal,
                DfaState::Terminal => DfaState::Initial,
            };
            if self.verbose {
                println!("char: {:?}, state: {:?} => {:?}", &ch, &current_state, &self.state);
            }
            if self.state == DfaState::Terminal {
                let content = self.content.to_owned();
                self.content = String::new();
                self.state = DfaState::Initial;
                Ok(Some(Token::from(current_state, content)))
            } else if self.state == DfaState::Initial {
                self.char_iter.next();
                Ok(None)
            } else {
                if let Some(ch) = self.char_iter.next() {
                    self.content.push(ch)
                }
                Ok(None)
            }
        } else {
            if !self.content.is_empty() && self.state != DfaState::Initial {
                let content = self.content.to_owned();
                self.content = String::new();
                Ok(Some(Token::from(self.state, content)))
            } else {
                Err(())
            }
        }
    }
}

impl<'a> Iterator for Laxer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.next_state() {
                Ok(Some(token)) => return Some(token),
                Err(_) => break,
                Ok(None) => continue
            }
        }
        None
    }
}
