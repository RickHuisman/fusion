mod compiler;
mod lexer;
mod parser;
mod vm;

use crate::vm::interpret;
use std::io;

fn main() {
    let source = r#"
    x = 5
    y = x / 2
    puts y
    puts x
    "#;
    interpret(source);
    // repl();
}

fn repl() {
    loop {
        let line = read_line();
        interpret(&line);
    }
}

fn read_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            trim_newline(&mut input);
            input
        }
        Err(error) => {
            panic!("error: {}", error);
        }
    }
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
