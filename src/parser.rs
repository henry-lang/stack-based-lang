use crate::error::CompileError;
use crate::tokenizer::{Spanned, Token};

static EOF: Spanned<Token> = Spanned::empty(Token::Eof);

#[derive(Debug)]
pub struct ParsedProgram {
    funcs: Vec<Spanned<ParsedFunc>>,
    entry_point: String,
}

#[derive(Debug)]
pub enum ParsedFunc {
    Named(String, Vec<Spanned<ParsedStatement>>),
    Unnamed(Vec<Spanned<ParsedStatement>>),
}

#[derive(Debug)]
pub enum ParsedStatement {
    PushNumber(i64),
    PushString(String),

    CallFunc(String),
}

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a [Spanned<Token>],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Spanned<Token>]) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn peek_token(&self) -> &'a Spanned<Token> {
        match self.tokens.get(self.pos) {
            Some(t) => t,
            None => &EOF,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.check()
    }

    fn advance(&mut self) -> &Spanned<Token> {}

    pub fn parse_named_func_decl(&mut self) -> Result<ParsedFunc, CompileError> {}

    pub fn parse(&mut self) -> Result<ParsedProgram, CompileError> {
        let mut funcs = vec![];
        let mut entry_point = None;

        while let Some(token) = self.peek() {
            match &token {
                FuncDeclName(name) => {}
                _ => {
                    return Err(CompileError::Spanned(
                        "expected a function declaration in the global scope".into(),
                        token.span,
                    ))
                }
            }
            break;
        }

        Ok(ParsedProgram {
            funcs,
            entry_point: match entry_point {
                Some(e) => e,
                None => {
                    return Err(CompileError::General(
                        "no entry point found for program, try adding \\main {}".into(),
                    ))
                }
            },
        })
    }
}
