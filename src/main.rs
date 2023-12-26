use std::io::{stdin, stdout};

mod lexer;
mod repl;

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");
    repl::start(&mut stdin(), &mut stdout());
}
