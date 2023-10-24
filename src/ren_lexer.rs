pub mod lexer {
    // charsets

    const CHS_SIGNS: [char; 2] = ['-', '+']; // or STATIC ?

    // public values

    pub struct State {
        pub mark: usize, // cursor position in original string
    }

    impl State {
        // functions

        pub fn init() -> Self {
            State { mark: 0 }
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
                    (c, i) if self.match_delimiter_char(c) => {
                        //State::set(&mut State {mark: Some(i)}, Some(i));
                    }, // end of word
                    _                            => {
                        //State::set(&mut State {mark:  None}, None);
                    },    // fail
                }
            }
            //State::set(&mut State {mark: Some(input.len())}, Some(input.len()));
        }

        pub fn match_integer(&mut self, input: &str) -> bool {
            println!("Starting at {:?}", self.mark);
            let string = &input[self.mark..];
            println!("Matching {string}");
            let mut count = 0;
            for (index, char) in string.chars().enumerate() {
                println!("Match at {index} = {char}");
                match (char, index) {
                    (c, 0) if CHS_SIGNS.contains(&c) => {count += 1}, // + or -
                    (c, _) if c.is_digit(10) => {count += 1},         // digit
                    (c, i) if self.match_delimiter_char(c) && count > 1 => {
                       self.mark += count;
                       println!("found delim at {i} and mark is {}", self.mark);
                       return true;
                    },                                      // after int
                    _ => {
                        println!("matched nothing!");
                        return false;
                    },                                      // fail
                }
            }
            self.mark += count;
            true
        }

        pub fn match_delimiter(&mut self, input: &str) -> bool {
            println!("Matching delimiter");
            let string = &input[self.mark..];
            if self.match_delimiter_char(string.chars().next().unwrap()) {
                println!("old mark {:?}", self.mark);
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
