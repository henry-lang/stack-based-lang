use crate::tokenizer::Span;
use ansi_term::Color::{Red, White};

use std::borrow::Cow;

#[derive(Debug)]
pub enum CompileError {
    General(Cow<'static, str>),
    Spanned(Cow<'static, str>, Span),
}

impl CompileError {
    pub fn message(&self) -> &Cow<'static, str> {
        match self {
            Self::General(msg) | Self::Spanned(msg, _) => msg,
        }
    }

    pub fn log_and_exit(&self, file: &str) -> ! {
        println!(
            "{}{}{}",
            Red.bold().paint("error"),
            White.bold().paint(": "),
            White.bold().paint(self.message().as_ref())
        );

        if let Self::Spanned(_, span) = self {
            println!("{}", file);
            println!("{:?}", span);
            println!("{}", &file[span.0..span.1]);
        }

        std::process::exit(1)
    }
}
