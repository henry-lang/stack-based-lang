use crate::tokenizer::Span;
use ansi_term::Color::{Red, White};

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
                println!(
                    "{}{}{}",
                    Red.bold().paint("error"),
                    White.bold().paint(": "),
                    White.bold().paint(msg.as_ref())
                )
            }
        }

        std::process::exit(1)
    }
}
