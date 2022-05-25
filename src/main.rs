mod tokenizer;

fn main() {
    let input = "\\name name";
    let tokens = tokenizer::tokenize(input);

    println!("{:?}", tokens);
}
