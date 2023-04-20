/// The cursor is used to iterate over the input string
/// It keeps track of the current position and the previous char
pub mod cursor {
    /// The cursor struct responsible for iterating over the input string
    /// It will keep track of the current position and the previous char
    #[derive(Clone)]
    pub struct Cursor {
        chars: Vec<char>,
        position: usize,
        previous: Option<char>,
    }

    impl Iterator for Cursor {
        type Item = char;

        fn next(&mut self) -> Option<Self::Item> {
            let prev = self.chars.get(self.position)?;
            self.previous = Some(*prev);
            self.position += 1;
            return Some(*prev);
        }

        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            let prev = self.chars.get(self.position)?;
            self.previous = Some(*prev);
            self.position += n + 1;
            return Some(*prev);
        }
    }

    /// The cursor iterator trait
    /// It is used to create a new cursor and move the cursor
    pub trait CursorIter {
        fn new(input: String) -> Cursor;
        fn peak_nth(&self, n: usize) -> Option<&[char]>;
        fn peek(&self) -> Option<&char>;
        fn position(&self) -> usize;
        fn previous(&self) -> Option<char>;
        fn advance_pos(&mut self, n: usize);
    }

    impl CursorIter for Cursor {
        /// Create a new cursor from a string
        fn new(input: String) -> Cursor {
            let chars = input.chars().collect();
            return Cursor {
                chars,
                position: 0,
                previous: None,
            };
        }

        /// Peak the next char this will not advance the position of the cursor therefore not
        /// consuming the char
        fn peak_nth(&self, n: usize) -> Option<&[char]> {
            if self.position + n >= self.chars.len() {
                return None;
            }
            return Some(&self.chars[self.position..(self.position + n)]);
        }

        /// Peak the next char this will not advance the position of the cursor therefore not
        /// consume the char
        fn peek(&self) -> Option<&char> {
            return Some(self.chars.get(self.position + 1)?);
        }

        /// Get the current position of the cursor
        fn position(&self) -> usize {
            return self.position;
        }

        /// Get the previous character
        fn previous(&self) -> Option<char> {
            return self.previous;
        }

        fn advance_pos(&mut self, n: usize) {
            self.position += n;
        }
    }
}
