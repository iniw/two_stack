use std::{iter::Peekable, str::CharIndices};
use thiserror::Error;

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

            c if c.is_ascii_digit() => {
                let number = {
                    let first_part = self.consume_while(start, |c| c.is_ascii_digit());
                    if let Some((second_start, _)) = self.src.next_if(|(_, c)| *c == '.') {
                        let second_part = self.consume_while(second_start, |c| c.is_ascii_digit());
                        self.src_data[start..(start + first_part.len() + second_part.len())]
                            .parse::<f64>()
                            .expect("Input number should be parseable into an f64")
                    } else {
                        first_part
                            .parse::<f64>()
                            .expect("Input number should be parseable into an f64")
                    }
                };

                Ok(Some(Token::Number(number)))
            }

            c if c.is_ascii_whitespace() => Ok(None),

            c => Err(LexerError::UnknownCharacter(c)),
        }
    }

    fn consume_while(&mut self, start: usize, f: impl Fn(char) -> bool) -> &'src str {
        while self.src.next_if(|(_, next)| f(*next)).is_some() {}
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

#[derive(Debug, Copy, Clone, Error)]
pub enum LexerError {
    #[error("Unknown character \"{0}\"")]
    UnknownCharacter(char),
}
