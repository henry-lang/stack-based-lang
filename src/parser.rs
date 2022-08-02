use crate::error::CompileError;
use crate::tokenizer::{Span, Spanned, Token, TokenKind};
use std::collections::HashMap;

static EOF: Spanned<Token> = Spanned::empty(Token::Eof);
static ENTRY_POINT: &str = "main";

type ParserResult<T> = Result<T, CompileError>;
type FuncMap = HashMap<String, Spanned<Func>>;

#[derive(Debug)]
pub struct Program {
    funcs: FuncMap,
}

impl Program {
    pub fn new(funcs: FuncMap) -> Self {
        Self { funcs }
    }

    pub fn funcs(&self) -> &FuncMap {
        &self.funcs
    }
}

#[derive(Debug)]
pub struct Func {
    statements: Vec<Spanned<Statement>>,
}

impl Func {
    pub fn new(statements: Vec<Spanned<Statement>>) -> Self {
        Self { statements }
    }

    pub fn statements(&self) -> &Vec<Spanned<Statement>> {
        &self.statements
    }
}

#[derive(Debug)]
pub enum Statement {
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

    fn consume(&mut self, kind: TokenKind) -> ParserResult<&Spanned<Token>> {
        let token = self.advance();

        if TokenKind::from(token) == kind {
            Ok(token)
        } else {
            Err(CompileError::Spanned(
                format!("expected {}, but found {}", kind, token.value).into(),
                token.span,
            ))
        }
    }

    fn consume_func_decl_name(&mut self) -> ParserResult<Spanned<String>> {
        let token = self.consume(TokenKind::FuncDeclName)?;

        if let Token::FuncDeclName(name) = &token.value {
            Ok(Spanned::new(name.clone(), token.span))
        } else {
            unreachable!();
        }
    }

    fn parse_number_literal(&mut self) -> ParserResult<Spanned<Statement>> {
        let token = self.consume(TokenKind::NumberLiteral)?;

        if let Token::NumberLiteral(num) = &token.value {
            Ok(Spanned::new(Statement::PushNumber(*num), token.span))
        } else {
            unreachable!();
        }
    }

    fn parse_string_literal(&mut self) -> ParserResult<Spanned<Statement>> {
        let token = self.consume(TokenKind::StringLiteral)?;

        if let Token::StringLiteral(string) = &token.value {
            Ok(Spanned::new(
                Statement::PushString(string.clone()),
                token.span,
            ))
        } else {
            unreachable!();
        }
    }

    fn parse_func_call(&mut self) -> ParserResult<Spanned<Statement>> {
        let token = self.consume(TokenKind::FuncName)?;

        if let Token::FuncName(string) = &token.value {
            Ok(Spanned::new(
                Statement::CallFunc(string.clone()),
                token.span,
            ))
        } else {
            unreachable!();
        }
    }

    fn parse_statement(&mut self) -> ParserResult<Spanned<Statement>> {
        match self.current_kind() {
            TokenKind::NumberLiteral => self.parse_number_literal(),
            TokenKind::StringLiteral => self.parse_string_literal(),
            TokenKind::FuncName => self.parse_func_call(),
            _ => Err(CompileError::Spanned(
                format!("expected a statement, but found {}. a statement is a literal or a function call",
                    self.current().value
                ).into(),
                self.current().span)
            )
        }
    }

    fn parse_named_func_decl(&mut self) -> ParserResult<(String, Spanned<Func>)> {
        let name = self.consume_func_decl_name()?;
        let mut statements = vec![];

        self.consume(TokenKind::OpenCurly)?;

        while !self.check(TokenKind::CloseCurly) {
            statements.push(self.parse_statement()?);
        }

        let end_span = self.consume(TokenKind::CloseCurly)?.span;

        Ok((
            name.value,
            Spanned::new(Func::new(statements), Span::unioned(name.span, end_span)),
        ))
    }

    pub fn parse(&mut self) -> ParserResult<Program> {
        let mut funcs = HashMap::new();

        while !self.is_eof() {
            let (name, func) = self.parse_named_func_decl()?;
            funcs.insert(name, func);
        }

        Ok(Program::new(funcs))
    }
}
