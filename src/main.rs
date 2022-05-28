mod error;
mod parser;
mod stack;
mod tokenizer;

use parser::ParsedProgram;

fn main() {
    let input = "\\name name";

    let tokens = tokenizer::tokenize(input).unwrap_or_else(|err| err.log_and_exit());
    println!("{:?}", tokens);

    let parsed_program = parser::Parser::new(&tokens[..])
        .parse()
        .unwrap_or_else(|err| err.log_and_exit());

    println!("{:?}", parsed_program);
}
