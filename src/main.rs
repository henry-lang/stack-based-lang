mod tokenizer;

fn main() {
    let input = "\\name 1 2 3 4 5 'Hello, world!'";
    let tokens = tokenizer::tokenize(input);

    println!("{:?}", tokens);
}
