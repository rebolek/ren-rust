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

        pub fn match_word(&mut self, input: &str) {
            for (index, char) in input.chars().enumerate() {
                match (char, index) {
                    (c, 0) if c.is_alphabetic()    => {}, // starts with alphabetic char
                    (c, _) if c.is_alphanumeric()  => {}, // continues with alphanumeric
                    (c, _) if self.match_delimiter_char(c) => {
                        //State::set(&mut State {mark: Some(i)}, Some(i));
                    }, // end of word
                    _                            => {
                        //State::set(&mut State {mark:  None}, None);
                    },    // fail
                }
            }
            //State::set(&mut State {mark: Some(input.len())}, Some(input.len()));
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
                    (c, 0) if c != '"' => { // is this really a string?
                        println!("no starting quotes");
                        return false;
                    },
                    ('"', 0) => { // match starting quotes
                        println!("starting quotes");
                    },
                    ('"', _) if is_escaped => { // escaped quotes, safely ignore
                        println!("escaped quotes");
                        self.content.push('"');
                        is_escaped = false;
                    },
                    ('"', i) if i > 0 => { // match ending quotes
                        self.mark += i + 1; // move after string
                        println!("ending quotes, mark at +{i}");
                        return true;
                    },
                    ('^', _) if !is_escaped => {
                        is_escaped = true;
                        println!("!!! found escape");
                    },
                    _ => {
                        if is_escaped {
                            is_escaped = false; // TODO: handle escapes
                        }
                        else {
                            self.content.push(char);
                        }
                        println!("string content");
                    },
                }
            }
            true
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
            println!("FIXME: This means there's no WS at end");
            true
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
}
