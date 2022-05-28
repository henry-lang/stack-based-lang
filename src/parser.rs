use crate::error::CompileError;
use crate::tokenizer::{Spanned, Token, TokenKind};

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

    fn current(&self) -> &'a Spanned<Token> {
        self.tokens.get(self.pos).unwrap_or_else(|| &EOF)
    }

    fn current_kind(&self) -> TokenKind {
        self.current().into()
    }

    fn is_eof(&self) -> bool {
        self.check(TokenKind::Eof)
    }

    fn check(&self, kind: TokenKind) -> bool {
        self.current_kind() == kind
    }

    fn advance(&mut self) -> &'a Spanned<Token> {
        let token = self.current();
        self.pos += 1;

        token
    }

    fn consume(&mut self, kind: TokenKind) -> Result<&Spanned<Token>, CompileError> {
        let token = self.advance();

        if TokenKind::from(token) == kind {
            Ok(token)
        } else {
            Err(CompileError::Spanned(
                format!("Expected {}, but found {}", kind, token.value).into(),
                token.span,
            ))
        }
    }

    pub fn parse(&mut self) -> Result<ParsedProgram, CompileError> {
        let mut funcs = vec![];
        let mut entry_point = None;

        while !self.is_eof() {
            self.consume(TokenKind::FuncDeclName)?;
        }

        Ok(ParsedProgram {
            funcs,
            entry_point: match entry_point {
                Some(e) => e,
                None => {
                    return Err(CompileError::General(
                        "No entry point found for program, try adding \\main {}".into(),
                    ))
                }
            },
        })
    }
}
