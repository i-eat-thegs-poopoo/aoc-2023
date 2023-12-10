use std::{iter::Peekable, str::CharIndices};

pub struct Parser<'a> {
    src: &'a str,
    pub inner: Peekable<CharIndices<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            inner: src.char_indices().peekable(),
        }
    }

    fn match_pattern(input: &mut impl Iterator<Item = (usize, char)>, pattern: &str) -> bool {
        for char in pattern.chars() {
            if input.next().filter(|(_, c)| *c == char).is_none() {
                return false;
            }
        }

        true
    }

    pub fn expect(&mut self, pattern: &str) {
        if !Self::match_pattern(&mut self.inner, pattern) {
            panic!("Failed to parse {pattern:?}");
        }
    }

    /// `true` if matches.
    pub fn consume_match(&mut self, pattern: &str) -> bool {
        let mut input = self.inner.clone();
        let matches = Self::match_pattern(&mut input, pattern);

        if matches {
            self.inner = input;
        }

        matches
    }

    pub fn str_while(&mut self, predicate: impl Fn(&char) -> bool) -> &'a str {
        let start = match self.inner.peek() {
            Some((start, char)) if predicate(char) => *start,
            _ => return "",
        };

        let end = loop {
            let idx = self.inner.next().unwrap().0;

            if self.inner.peek().filter(|(_, c)| predicate(c)).is_none() {
                break idx + 1;
            }
        };

        &self.src[start..end]
    }

    pub fn int(&mut self) -> u64 {
        self.str_while(char::is_ascii_digit).parse().unwrap()
    }

    pub fn ident(&mut self) -> &'a str {
        self.str_while(char::is_ascii_alphabetic)
    }

    pub fn alphanumeric(&mut self) -> &'a str {
        self.str_while(char::is_ascii_alphanumeric)
    }

    /// Expects at least one item.
    pub fn sep_by(&mut self, sep: &str, mut callback: impl FnMut(&mut Self)) {
        callback(self);

        loop {
            let mut input = self.inner.clone();

            if !Self::match_pattern(&mut input, sep) {
                return;
            }

            self.inner = input;
            callback(self);
        }
    }
}
