use rust_dsl::lexer::tokenize;

fn main() {
    println!("Welcome to your Racket interpreter!");
    
    let input = "(+ 1 2)";
    let result = tokenize(input);

    println!("Tokenize result: {:?}", result);
}
