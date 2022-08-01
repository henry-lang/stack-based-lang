use crate::error::CompileError;
use std::cmp;

#[derive(Debug, Copy, Clone)]
pub struct Span(pub usize, pub usize);

impl Span {
    pub fn unioned(a: Self, b: Self) -> Self {
        Self(cmp::min(a.0, b.0), cmp::max(a.1, b.1))
    }

    pub fn len(&self) -> usize {
        self.1 - self.0
    }
}

#[derive(Debug)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }

    pub const fn empty(value: T) -> Self {
        Self {
            value,
            span: Span(0, 0),
        }
    }
}

#[derive(Debug)]
pub enum Token {
    NumberLiteral(i64),
    StringLiteral(String),
    FuncName(String),

    FuncDeclName(String), // Example: \function
    OpenCurly,
    CloseCurly,

    Eof,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind: TokenKind = self.into();

        write!(f, "{}", kind)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum TokenKind {
    NumberLiteral,
    StringLiteral,
    FuncName,

    FuncDeclName,
    OpenCurly,
    CloseCurly,

    Eof,
}

impl From<&Token> for TokenKind {
    fn from(token: &Token) -> Self {
        match token {
            Token::NumberLiteral(_) => Self::NumberLiteral,
            Token::StringLiteral(_) => Self::StringLiteral,
            Token::FuncName(_) => Self::FuncName,

            Token::FuncDeclName(_) => Self::FuncDeclName,
            Token::OpenCurly => Self::OpenCurly,
            Token::CloseCurly => Self::CloseCurly,
            Token::Eof => Self::Eof,
        }
    }
}

impl From<&Spanned<Token>> for TokenKind {
    fn from(spanned: &Spanned<Token>) -> Self {
        TokenKind::from(&spanned.value)
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NumberLiteral => "number",
                Self::StringLiteral => "string",
                Self::FuncName => "func name",

                Self::FuncDeclName => "func decl name",
                Self::OpenCurly => "{",
                Self::CloseCurly => "}",

                Self::Eof => "<eof>",
            }
        )
    }
}

pub fn tokenize(source: &str) -> Result<Vec<Spanned<Token>>, CompileError> {
    let mut tokens = vec![];

    let mut iter = source.chars().enumerate().peekable();

    while let Some((i, c)) = iter.next() {
        tokens.push(match c {
            '0'..='9' => {
                let mut end = i + 1;
                while iter.next_if(|(_, next)| !next.is_whitespace()).is_some() {
                    end += 1;
                }

                let value = source[i..end].parse::<i64>();
                match value {
                    Ok(num) => Spanned::new(Token::NumberLiteral(num), Span(i, end)),
                    Err(_) => {
                        return Err(CompileError::Spanned(
                            "invalid number literal".into(),
                            Span(i, end),
                        ))
                    }
                }
            }
            '"' | '\'' => {
                let mut end = i + 1;
                while iter.next_if(|(_, next)| *next != c).is_some() {
                    end += 1;
                }

                if iter.next_if(|(_, next)| *next == c).is_none() {
                    // End of string
                    return Err(CompileError::Spanned(
                        "found end of file while parsing string".into(),
                        Span(i, end),
                    ));
                }

                Spanned::new(
                    Token::StringLiteral(source[i + 1..end].into()),
                    Span(i, end + 1),
                )
            }
            '\\' => {
                let mut end = i + 1;

                while let Some(_) = iter.next_if(|(_, next)| !next.is_whitespace()) {
                    // TODO: ensure function name is valid
                    end += 1;
                }

                if i + 1 == end {
                    return Err(CompileError::Spanned(
                        "function name is required".into(),
                        Span(i, end),
                    ));
                }

                Spanned::new(Token::FuncDeclName(source[i + 1..end].into()), Span(i, end))
            }
            '{' => Spanned::new(Token::OpenCurly, Span(i, i + 1)),
            '}' => Spanned::new(Token::CloseCurly, Span(i, i + 1)),
            ' ' | '\t' | '\n' | '\r' => continue,
            _ => {
                let mut end = i + 1;

                while let Some(_) = iter.next_if(|(_, next)| !next.is_whitespace()) {
                    end += 1;
                }

                Spanned::new(Token::FuncName(source[i..end].into()), Span(i, end))
            }
        });
    }

    Ok(tokens)
}
