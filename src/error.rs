use crate::tokenizer::Span;

use std::borrow::Cow;

#[derive(Debug)]
pub enum CompileError {
    General(Cow<'static, str>),
    Spanned(Cow<'static, str>, Span),
}

impl CompileError {
    pub fn log_and_exit(&self) -> ! {
        match self {
            Self::General(_) => {}
            Self::Spanned(msg, _) => {
                println!("{}", msg)
            }
        }

        std::process::exit(1)
    }
}
