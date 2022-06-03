mod error;
mod parser;
mod runtime;
mod stack;
mod tokenizer;

use std::env;
use std::fs;

use error::CompileError;

fn main() {
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
    let mut program = parser
        .parse()
        .unwrap_or_else(|err| err.log_and_exit(&input));

    println!("{:?}", program)
}
