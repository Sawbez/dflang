use std::{env, fs};

mod lang;

use self::lang::lexer::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let source = fs::read_to_string(&args[1]).expect("cannot read file");

    println!("Contents:\n-----\n{}\n-----\n", source);

    let lexer = Lexer::new(&source);
    for tok in lexer {
        println!("{:?}", tok);
    }
}
