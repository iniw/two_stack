use core::fmt;
use std::{fmt::Display, iter::Peekable, str::CharIndices};

#[derive(Debug, Clone)]
pub struct Lexer<'src> {
    src_data: &'src str,
    src: Peekable<CharIndices<'src>>,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            src_data: src,
            src: src.char_indices().peekable(),
        }
    }

    pub fn lex(mut self) -> (Vec<Token>, Vec<LexerError>) {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        while let Some((start, c)) = self.src.next() {
            match self.interpret_char(c, start) {
                Ok(Some(token)) => tokens.push(token),
                Ok(None) => (),
                Err(err) => errors.push(err),
            }
        }

        (tokens, errors)
    }

    fn interpret_char(&mut self, c: char, start: usize) -> Result<Option<Token>, LexerError> {
        match c {
            '+' => Ok(Some(Token::Operator(Operator::Add))),
            '/' => Ok(Some(Token::Operator(Operator::Div))),
            '*' => Ok(Some(Token::Operator(Operator::Mul))),
            '-' => Ok(Some(Token::Operator(Operator::Sub))),

            '(' => Ok(Some(Token::Operator(Operator::LeftParenthesis))),
            ')' => Ok(Some(Token::Operator(Operator::RightParenthesis))),

            c if c.is_ascii_whitespace() => Ok(None),

            c if c.is_ascii_digit() => {
                let number = {
                    let number_str = {
                        let pre_dot = self.consume_while(start, char::is_ascii_digit);
                        if let Some((dot_pos, _)) = self.src.next_if(|(_, c)| *c == '.') {
                            let post_dot = self.consume_while(dot_pos, char::is_ascii_digit);
                            &self.src_data[start..start + pre_dot.len() + post_dot.len()]
                        } else {
                            pre_dot
                        }
                    };
                    number_str
                        .parse::<f64>()
                        .expect("Input number should be parseable into an f64")
                };

                Ok(Some(Token::Number(number)))
            }

            c => Err(LexerError::UnknownCharacter(c)),
        }
    }

    fn consume_while(&mut self, start: usize, f: impl Fn(&char) -> bool) -> &'src str {
        while self.src.next_if(|(_, next)| f(next)).is_some() {}
        self.current_lexeme_starting_from(start)
    }

    fn current_lexeme_starting_from(&mut self, start: usize) -> &'src str {
        if let Some((end, _)) = self.src.peek() {
            &self.src_data[start..*end]
        } else {
            &self.src_data[start..]
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Operator(Operator),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    Add,
    Div,
    Mul,
    Sub,
    LeftParenthesis,
    RightParenthesis,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Low,
    High,
}

impl Operator {
    pub fn precedence(&self) -> Option<Precedence> {
        match self {
            Operator::Add => Some(Precedence::Low),
            Operator::Div => Some(Precedence::High),
            Operator::Mul => Some(Precedence::High),
            Operator::Sub => Some(Precedence::Low),
            Operator::LeftParenthesis => None,
            Operator::RightParenthesis => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum LexerError {
    UnknownCharacter(char),
}

impl Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnknownCharacter(c) => write!(f, "Unknown character \"{c}\""),
        }
    }
}
