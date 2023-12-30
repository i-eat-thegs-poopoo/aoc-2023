use crate::grid::*;
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

    pub fn next(&mut self) -> Option<char> {
        self.inner.next().map(|(_, c)| c)
    }

    pub fn peek(&mut self) -> Option<char> {
        self.inner.peek().map(|(_, c)| *c)
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

    pub fn signed_int(&mut self) -> i64 {
        let neg = self.consume_match("-");
        let num = self.str_while(char::is_ascii_digit).parse::<i64>().unwrap();

        if neg {
            -num
        } else {
            num
        }
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

    /// Expects at least one item.
    pub fn sep_by_until(&mut self, sep: &str, until: &str, mut callback: impl FnMut(&mut Self)) {
        callback(self);

        loop {
            if self.consume_match(until) || !self.consume_match(sep) {
                return;
            }

            callback(self);
        }
    }

    pub fn grid<T>(&mut self, mut parse_tile: impl FnMut(char) -> T) -> Grid<T> {
        let mut tiles = Vec::new();
        let mut curr_row = Vec::new();

        while let Some(char) = self.peek() {
            if char == '\n' {
                if curr_row.is_empty() {
                    break;
                } else {
                    tiles.push(curr_row);
                    curr_row = Vec::new();
                }
            } else {
                curr_row.push(parse_tile(char));
            }

            self.next();
        }

        if !curr_row.is_empty() {
            tiles.push(curr_row);
        }

        Grid { tiles }
    }

    pub fn grid_with_pos<T>(
        &mut self,
        mut parse_tile: impl FnMut(char, (usize, usize)) -> T,
    ) -> Grid<T> {
        let mut tiles = Vec::new();
        let mut curr_row = Vec::new();

        let mut row = 0;
        let mut col = 0;

        while let Some(char) = self.peek() {
            if char == '\n' {
                if curr_row.is_empty() {
                    break;
                } else {
                    tiles.push(curr_row);
                    curr_row = Vec::new();
                    row = 0;
                    col += 1;
                }
            } else {
                curr_row.push(parse_tile(char, (row, col)));
                row += 1;
            }

            self.next();
        }

        if !curr_row.is_empty() {
            tiles.push(curr_row);
        }

        Grid { tiles }
    }
}
