mod error;
mod parser;
mod stack;
mod tokenizer;

fn main() {
    let input = "\\name name";

    let tokens = tokenizer::tokenize(input).unwrap(); // TODO: Handle error here in a more graceful way
    println!("{:?}", tokens);

    let parsed_program = parser::Parser::new(&tokens).parse().unwrap();
    println!("{:?}", parsed_program);
}
