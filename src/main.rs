use std::io::{self, Write};

pub mod types;
pub mod ren_lexer;

use types::Value;
use ren_lexer::State;

// structs and stuff

#[derive(Debug)]
pub enum ReplState {
    Normal,
    Debug,
    Step,
}

#[derive(Debug)]
pub struct Repl {
    pub state: ReplState,
}

impl Repl {
    pub fn make(state: ReplState) -> Repl {
        Repl {
            state, 
        }
    }

    pub fn set_state(&mut self, state: ReplState) {
        self.state = state;
    }
}


// functions

fn parse_line(input: &str, session: &Repl) -> Vec<Value> {
    let mut mark = 0;
    let mut break_input = String::new();
    let mut word_lexer = State::init();
    loop {

        println!("AT index {mark}");

        word_lexer.match_value(&input);
        mark = word_lexer.get();

        let len = input.len();
        println!("len: {len}, mark:{:?}", word_lexer.get());

        println!("{:?}", session.state);

        match session.state {
            ReplState::Debug | ReplState::Step => {
                io::stdin()
                    .read_line(&mut break_input)
                    .expect("Failed to read line");
            },
            _ => {},
        }

        println!("len={len}, mark={mark}");

        // end condition
        if len == mark {
            break;
        }

    }
    println!("Block stack has {} items", word_lexer.block_stack.len());
    word_lexer.values
}

fn repl(session: &Repl) {
    loop {
        // print prompt
        print!(">> ");
        io::stdout().flush().expect("Failed to flush");

        // handle input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // parse input
        let result;
        if input.chars().next() == Some('q') {
            // FIXME: hardcoded end of repl
            break
        } else {
            result = parse_line(&input, &session);
        }
        println!("Found these values: {:?}", result);
    }
}

fn welcome() -> Option<Repl> {
    println!("
Welcome to Ren parser.
Select mode:

1) press Enter for normal mode
2) type \"debug\" to display debug info
3) type \"step\" to display debug info and enter step mode
4) type \"quit\" to exit

You can use first letters instead of full words.
");
    print!("-> ");
    io::stdout().flush().expect("Failed to flush");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.pop();
    let inp: &str = &input;

    match inp {
        "d" | "debug" => {Some(Repl::make(ReplState::Debug))},
        "s" | "step"  => {Some(Repl::make(ReplState::Step))},
        "q" | "quit"  => {None},
        _             => {Some(Repl::make(ReplState::Normal))},
    }
}

fn main() {
    let session = welcome();
    println!("session type: {:?}", session);
    match session {
        Some(value) => {repl(&value);},
        None => {},
    }
}


