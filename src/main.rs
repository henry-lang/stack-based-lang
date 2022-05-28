mod error;
mod parser;
mod stack;
mod tokenizer;

fn main() {
    let input = "\\ name";

    let tokens = tokenizer::tokenize(input).unwrap_or_else(|err| err.log_and_exit());

    let parsed_program = parser::Parser::new(&tokens[..])
        .parse()
        .unwrap_or_else(|err| err.log_and_exit());

    println!("{:?}", parsed_program);
}
