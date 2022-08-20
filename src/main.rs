#![feature(int_log)]
#![feature(path_file_prefix)]

mod codegen;
mod error;
mod parser;
mod tokenizer;

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use ansi_term::Color::{Green, White};
use error::CompileError;

fn main() {
    let start_time = std::time::Instant::now();

    let input_file = env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "main.sbl".into());

    let input = fs::read_to_string(&input_file);
    let input = input.unwrap_or_else(|_| {
        CompileError::General(format!("can't open file {input_file}").into()).log_and_exit("")
    });

    let tokens = tokenizer::tokenize(&input).unwrap_or_else(|err| err.log_and_exit(&input));

    let mut parser = parser::Parser::new(&tokens[..]);
    let program = parser
        .parse()
        .unwrap_or_else(|err| err.log_and_exit(&input));

    let compiled = codegen::Codegen::new(&program)
        .gen()
        .unwrap_or_else(|err| err.log_and_exit(&input));

    let out_file = format!(
        "{}.c",
        Path::new(&input_file)
            .file_prefix()
            .unwrap_or(OsStr::new("main"))
            .to_str()
            .unwrap_or("main")
    );
    fs::write(&out_file, compiled).expect("write file"); // TODO: Handle file error better

    println!(
        "{} compilation of {} into {} in {}ms",
        Green.bold().paint("finished"),
        White.bold().paint(input_file),
        White.bold().paint(out_file),
        start_time.elapsed().as_millis()
    );
}
