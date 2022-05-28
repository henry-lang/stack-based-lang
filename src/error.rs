use crate::tokenizer::Span;

use std::borrow::Cow;

#[derive(Debug)]
pub enum CompileError {
    General(Cow<'static, str>),
    Spanned(Cow<'static, str>, Span),
}
