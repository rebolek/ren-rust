use crate::types;
use crate::types::Value;
use crate::types::RenType;
use crate::types::ValueType;

// charsets

const CHS_SIGNS: [char; 2] = ['-', '+']; // or STATIC ?

// public values

pub struct State {
    pub mark: usize, // cursor position in original string
    pub content: String,
    pub values: Vec<Value>,
    pub block_stack: Vec<Vec<Value>>,
}

impl State {
    // functions

    pub fn init() -> Self {
        State {
            mark: 0,
            content: "".to_string(),
            values: Vec::new(),
            block_stack: Vec::new(),
        }
    }

    pub fn clear_content(&mut self) {
        self.content = "".to_string();
    }

    pub fn get(&self) -> usize {
        self.mark
    }

    pub fn set(&mut self, value: usize) {
        self.mark = value;
    }

    // main matcher
    pub fn match_value(&mut self, input: &str) {
        println!("MARK: {}, INPUT: '{}'", self.mark, &input[self.mark..]);
        if self.match_block(&input) {
            println!("BLOCK");
        } else
        if self.match_integer(&input) {
            let value = Value::convert(RenType::Integer, self.content.to_string());
            self.values.push(value);
        } else
        if self.match_string(&input) {
            let value = Value::convert(RenType::String, self.content.to_string());
            self.values.push(value);
        } else
        if self.match_word(&input) {
            let value = Value::convert(RenType::Word, self.content.to_string());
            self.values.push(value);
        } else
        if self.match_whitespace(&input) {
        }
    }

    // submatchers

    fn match_word(&mut self, input: &str) -> bool {
        println!("MATCH: word");
        let string = &input[self.mark..];
        self.clear_content();
        for (index, char) in string.chars().enumerate() {
            println!("checking {char} at {index}");
            match (char, index) {
                (c, 0) if c.is_alphabetic() => {
                    // starts with alphabetic char
                    self.mark += 1;
                    self.content.push(c);
                },
                (c, i) if c.is_alphanumeric() && i > 0 => {
                    // continues with alphanumeric
                    self.mark += 1;
                    self.content.push(c);
                },
                (c, i) if self.match_delimiter_char(c) && i > 0 => {
                    println!("delim matched");
                    // end of word
                    return true;
                },
                _ => {
                    println!("WORD:forbidden");
                    // forbidden char
                    return false;
                },
            }
        }
        println!("NOTE: reached end, look at this!");
        false
    }

    fn match_string(&mut self, input: &str) -> bool {
        Self::match_inline_string(self, &input)
    }

    fn match_inline_string(&mut self, input: &str) -> bool {
        println!("MATCH: string-inline");
        let string = &input[self.mark..];
        let mut is_escaped = false;
        self.clear_content();
        for (index, char) in string.chars().enumerate() {
            println!("checking {char} at {index}");
            match (char, index) {
                (c, 0) if c != '"' => {          // is this really a string?
                    println!("no starting quotes");
                    return false;
                },
                ('"', 0) => {}, // match starting quotes
                ('"', _) if is_escaped => {     // escaped quotes, safely ignore
                    self.content.push('"');
                    is_escaped = false;
                },
                ('"', i) if i > 0 => {          // match ending quotes
                    self.mark += i + 1; // move after string
                    return true;
                },
                ('^', _) if !is_escaped => {    // start escape sequence
                    is_escaped = true;
                },
                ('(', _) if is_escaped => {
                    // TODO: process escapes in parens
                }
                _ => {
                    if is_escaped {
                        is_escaped = false;
                        self.content.push(process_escape(char));
                    }
                    else {
                        self.content.push(char);
                    }
                },
            }
        }
        println!("NOTE: reached end, look at this!");
        false
    }

    fn match_integer(&mut self, input: &str) -> bool {
        println!("MATCH: integer");
        let string = &input[self.mark..];
        self.clear_content();
        for (index, char) in string.chars().enumerate() {
            println!("checking {char} at {index}");
            match (char, index) {
                (c, 0) if CHS_SIGNS.contains(&c) => {self.content.push(c)}, // + or -
                (c, _) if c.is_digit(10) => {self.content.push(c)}, // digit
                (c, i) if self.match_delimiter_char(c) && i > 1 => {
                   self.mark += i;
                   return true;
                },                                      // after int
                _ => {
                    return false;
                },                                      // fail
            }
        }
        println!("NOTE: reached end, look at this!");
        false
    }

    fn match_block(&mut self, input: &str) -> bool {
        println!("MATCH: block");
        let string = &input[self.mark..];
        self.clear_content();
        for (index, char) in string.chars().enumerate() {
            println!("checking {char} at {index}");
            match (char, index) {
                ('[', 0) => {
                    // block start matched

                    self.mark += 1;

                    //self.block_stack.push(self.values);
                    self.block_stack.push(std::mem::take(&mut self.values));
                    self.values = Vec::new();

                    return true;
                },
                (']', _) => {
                    self.mark += 1;

                    let len = self.block_stack.len();
                    println!("block stack has {len} values");
                    let mut last_block = self.block_stack.pop().expect("No block on stack");
                    last_block.push( types::Value { value: ValueType::Block(std::mem::take (&mut self.values)) } );
                    self.values = last_block;

                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
        println!("NOTE: reached end, look at this!");
        false
    }

    fn match_whitespace(&mut self, input: &str) -> bool {
        println!("MATCH: whitespace");
        let string = &input[self.mark..];
        for (index, char) in string.chars().enumerate() {
            println!("checking {char} at {index}");
            if char.is_whitespace() {
                println!("WS matched");
                self.mark += 1;
                return true;
            }
        }
        false
    }

    fn match_delimiter(&mut self, input: &str) -> bool {
        let string = &input[self.mark..];
        if self.match_delimiter_char(string.chars().next().unwrap()) {
            self.mark += 1;
            true
        } else {
            false
        }
    }

    fn match_delimiter_char(&mut self, input: char) -> bool {
        if input.is_whitespace()
        || input == '[' || input == '(' || input == '{' 
        || input == ']' || input == ')' || input == '}' {
            true
        } else {
            // TODO: check for other delimiters
            false
        }
    }
}
fn process_escape(input: char) -> char {
    match input {
        '/' => {'\n'},
        '-' => {'\t'},
        _   => {input},
    }
}
