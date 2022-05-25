#[derive(Debug)]
pub struct Span(usize, usize);

#[derive(Debug)]
pub struct Token {
    contents: TokenContents,
    span: Span,
}

#[derive(Debug)]
pub enum TokenContents {
    NumberLiteral(i64),
    StringLiteral(String),

    FuncName(String), // Example: \function
    OpenCurly,
    CloseCurly,
}

#[derive(Debug)]
pub struct TokenizeError {
    message: String,
    span: Span,
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, TokenizeError> {
    let mut tokens = vec![];
    let mut line_num = 0usize;

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
                    Ok(num) => Token {
                        contents: TokenContents::NumberLiteral(num),
                        span: Span(i, end),
                    },
                    Err(_) => {
                        return Err(TokenizeError {
                            message: "invalid number literal".into(),
                            span: Span(i, end),
                        })
                    }
                }
            }
            '"' | '\'' => {
                let mut end = i + 1; // Skip first quote
                while iter.next_if(|(_, next)| *next != c).is_some() {
                    end += 1;
                }

                if iter.next_if(|(_, next)| *next == c).is_none() {
                    // End of string
                    return Err(TokenizeError {
                        message: "found end of file while parsing string".into(),
                        span: Span(i, end),
                    });
                }

                Token {
                    contents: TokenContents::StringLiteral(source[i + 1..end].into()),
                    span: Span(i, end),
                }
            }
            '\\' => {
                let mut end = i + 1;

                while let Some((_, next)) = iter.next_if(|(_, next)| !next.is_whitespace()) {
                    // TODO: ensure function name is valid
                    end += 1;
                }

                if iter.next_if(|(_, next)| next.is_whitespace()).is_none() {
                    // End of string
                    return Err(TokenizeError {
                        message: "found end of file while parsing function name".into(),
                        span: Span(i, end),
                    });
                }

                if i + 1 == end {
                    return Err(TokenizeError {
                        message: "function name is required".into(),
                        span: Span(i, end),
                    });
                }

                Token {
                    contents: TokenContents::FuncName(source[i + 1..end].into()),
                    span: Span(i, end),
                }
            }
            '{' => Token {
                contents: TokenContents::OpenCurly,
                span: Span(i, i),
            },
            '}' => Token {
                contents: TokenContents::CloseCurly,
                span: Span(i, i),
            },
            '\n' => {
                line_num += 1;
                continue;
            }
            ' ' | '\t' => continue,
            _ => continue,
        });
    }

    Ok(tokens)
}
