use std::io::{self, Write};

pub mod ren_lexer;

use crate::ren_lexer::lexer;

#[derive(Debug)]
enum RenType {
    Integer,
    String,
    Word,
}

#[derive(Debug)]
enum ValueType {
    Integer     (i32),
    String      (String),
    Word        (String),
}

#[derive(Debug)]
struct Value {
    value: ValueType,
}

impl Value {
    fn make(value: ValueType) -> Self {
        Value {
            value,
        }
    }

    fn convert(
        ren_type: RenType,
        content: String,
    )   -> Value {
        let val: ValueType = match ren_type {
            RenType::Integer => {
                ValueType::Integer(content.parse::<i32>().unwrap())
            },
            RenType::String => {
                ValueType::String(content)
            },
            RenType::Word => {
                ValueType::Word(content)
            },
            _ => {
                ValueType::String(content)
            },
        };
        Value::make(
            //ren_type.to_string(),
            val,
        )
    }
}

// functions

fn parse_line(input: &str) -> Vec<Value> {
    let mut mark = 0;
    let mut values = Vec::new();
    let mut break_input = String::new();
    let mut word_lexer = lexer::State::init();
    loop {
        let start = mark;

        println!("AT index {start}");

        if word_lexer.match_integer(&input) {
            mark = word_lexer.get();
            let value = Value::convert(RenType::Integer, word_lexer.content.to_string());
            values.push(value);
        } else
        if word_lexer.match_string(&input) {
            mark = word_lexer.get();
            println!("STRcontent: {}", &word_lexer.content);
            let value = Value::convert(RenType::String, word_lexer.content.to_string());
            values.push(value);
        } else
        if word_lexer.match_word(&input) {
            mark = word_lexer.get();
            println!("WRD:content: {}", &word_lexer.content);
            let value = Value::convert(RenType::Word, word_lexer.content.to_string());
            values.push(value);
        } else
        if word_lexer.match_delimiter(&input) {
            mark = word_lexer.get();
        }

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


