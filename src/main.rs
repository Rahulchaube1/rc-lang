mod interpreter;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: rc-lang <filename>");
        return;
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|l| l.to_string()).collect();

    interpreter::interpret(lines);
}
