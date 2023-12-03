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

    pub fn expect(&mut self, pattern: &str) {
        for char in pattern.chars() {
            if self.inner.next().filter(|(_, c)| *c == char).is_none() {
                panic!("Failed to parse {pattern:?}");
            } 
        }
    }

    fn span_while(&mut self, predicate: impl Fn(&char) -> bool) -> &'a str {
        let start = self.inner.peek().unwrap().0;
        let end = loop {
            let idx = self.inner.next().unwrap().0;

            if self.inner.peek().filter(|(_, c)| predicate(c)).is_none() {
                break idx + 1;
            }
        };

        &self.src[start..end]
    }

    pub fn int(&mut self) -> u32 {
        self.span_while(char::is_ascii_digit).parse().unwrap()
    }

    pub fn ident(&mut self) -> &'a str {
        self.span_while(char::is_ascii_alphabetic)
    }

    /// Expects at least one item.
    pub fn sep_by(&mut self, sep: &str, mut callback: impl FnMut(&mut Self)) {
        callback(self);

        loop {
            let mut input = self.inner.clone();

            for char in sep.chars() {
                if input.next().filter(|(_, c)| *c == char).is_none() {
                    return;
                }
            }

            self.inner = input;
            callback(self);
        }
    }
}
