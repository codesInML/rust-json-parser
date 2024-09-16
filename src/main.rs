use std::{env, fs::File, io::Read, process};

use json_parser::{Lexer, Parser};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1);
    let file;

    match filename {
        Some(name) => file = name,
        None => panic!("file path not provided"),
    }

    let mut content = String::new();
    let mut file = File::open(file).expect("could not open file");
    file.read_to_string(&mut content)
        .expect("could not read file");

    let mut lexer = Lexer::new(content);
    lexer.tokenize();

    println!("{:#?}", lexer.get_tokens());

    let mut parser = Parser::new(lexer.get_tokens());
    match parser.parse() {
        Ok(code) => {
            println!("successfully parsed JSON file");
            process::exit(code);
        }
        Err(msg) => {
            println!("{msg}");
            process::exit(2);
        }
    }
}
