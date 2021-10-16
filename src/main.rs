use std::io;

fn main() {
    repl();
}

fn repl() {
    loop {
        println!("{}", &read_line());
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
