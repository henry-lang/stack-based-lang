use crate::tokenizer::Span;
use ansi_term::Color::{Blue, Red, White};

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
            let line_num = file[..span.0].chars().filter(|x| *x == '\n').count();
            let padding = (line_num.checked_log10().unwrap_or(0) + 4) as usize + span.0;
            println!(
                "{} {} {}",
                Blue.bold().paint(line_num.to_string()),
                Blue.bold().paint("|"),
                file.lines().nth(line_num).unwrap()
            );
            println!(
                "{}{}",
                " ".repeat(padding),
                Red.bold().paint("^".repeat(span.len()))
            );
        }

        std::process::exit(1)
    }
}
