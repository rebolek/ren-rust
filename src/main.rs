use std::io::{self, Write};

pub mod types;
pub mod ren_lexer;

use types::Value;
use ren_lexer::State;

// functions

fn parse_line(input: &str) -> Vec<Value> {
    let mut mark = 0;
    let mut values: Vec<Value> = Vec::new();
    let mut break_input = String::new();
    let mut word_lexer = State::init();
    let mut target = values;
    loop {

        println!("AT index {mark}");

        word_lexer.match_value(&input, &mut target);
        mark = word_lexer.get();

        let len = input.len();
        println!("len: {len}, mark:{:?}", word_lexer.get());

        io::stdin()
            .read_line(&mut break_input)
            .expect("Failed to read line");

        // end condition
        if len == mark {
            break;
        }

    }
    let values = target;
    values
}

fn repl() {
    loop {
        // print prompt
        print!(">> ");
        io::stdout().flush().expect("Failed to flush");

        // handle input
        let mut input = String::new();
        let result;

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // parse input
        if input.chars().next() == Some('q') {
            // FIXME: hardcoded end of repl
            break
        } else {
            result = parse_line(&input);
        }
        println!("Found these values: {:?}", result);
    }
}

fn main() {
    repl();
}


