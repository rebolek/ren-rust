pub mod lexer {
    // charsets

    const CHS_SIGNS: [char; 2] = ['-', '+']; // or STATIC ?

    // public values

    pub struct State {
        pub mark: usize, // cursor position in original string
        pub content: String,
    }

    impl State {
        // functions

        pub fn init() -> Self {
            State {
                mark: 0,
                content: "".to_string(),
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

        pub fn match_word(&mut self, input: &str) -> bool {
            let string = &input[self.mark..];
            self.clear_content();
            for (index, char) in string.chars().enumerate() {
                println!("checking {char} at {index}");
                match (char, index) {
                    (c, 0) if c.is_alphabetic() => {
                        // starts with alphabetic char
                        self.content.push(c);
                    },
                    (c, i) if c.is_alphanumeric() && i > 0 => {
                        // continues with alphanumeric
                        self.content.push(c);
                    },
                    (c, i) if self.match_delimiter_char(c) && i > 0 => {
                        // end of word
                        self.mark += i;
                        return true;
                    },
                    _ => {
                        println!("forbidden");
                        // forbidden char
                        return false;
                    },
                }
            }
        }

        pub fn match_string(&mut self, input: &str) -> bool {
            Self::match_inline_string(self, &input)
        }

        fn match_inline_string(&mut self, input: &str) -> bool {
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
        }

        pub fn match_integer(&mut self, input: &str) -> bool {
            let string = &input[self.mark..];
            self.clear_content();
            for (index, char) in string.chars().enumerate() {
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
        }

        pub fn match_delimiter(&mut self, input: &str) -> bool {
            let string = &input[self.mark..];
            if self.match_delimiter_char(string.chars().next().unwrap()) {
                self.mark += 1;
                true
            } else {
                false
            }
        }

        fn match_delimiter_char(&mut self, input: char) -> bool {
            if input.is_whitespace() {
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
}
