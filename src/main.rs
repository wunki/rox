use std::{
    fs::read_to_string,
    io::{self, Write},
    process::exit,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        println!("Usage: rox <filename>");
        exit(64);
    } else if args.len() == 2 {
        run_file(args[1].clone());
    } else {
        run_prompt();
    }
}

/// Reads the whole file and runs it.
fn run_file(filename: String) {
    let source = read_to_string(filename).expect("Failed to read file.");
    run(source);
}

/// Runs an interactive prompt in a loop.
fn run_prompt() {
    println!("Welcome to Rox! (Hit Ctrl-C to exit).");
    println!("Hold on to your socks! Here we go!");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut source = String::new();
        std::io::stdin()
            .read_line(&mut source)
            .expect("Failed to read line.");
        run(source);
    }
}

/// The meat and potatoes of the interpreter.
fn run(source: String) {
    println!("{}", source);
}
