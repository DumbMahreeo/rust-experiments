use std::{
    fmt::{Display, Write},
    iter::{Map, Skip, StepBy, Zip},
};

pub struct U8Iter {
    val: u8,
    counter: u8,
}

impl U8Iter {
    pub fn new(val: u8) -> Self {
        U8Iter { val, counter: 0 }
    }

    /// Creates an iterator that consumes two bits every iteration
    pub fn into_pairs(
        &self,
    ) -> std::iter::Zip<std::iter::StepBy<U8Iter>, std::iter::StepBy<std::iter::Skip<U8Iter>>> {
        self.clone().step_by(2).zip(self.clone().skip(1).step_by(2))
    }

    /// Creates an iterator that consumes two bits every iteration and converts them to Morse characters
    /// following spec.md
    pub fn into_morse(
        &self,
    ) -> Map<Zip<StepBy<U8Iter>, StepBy<Skip<U8Iter>>>, impl FnMut((bool, bool)) -> Morse> {
        use Morse::*;
        let a = self.into_pairs().map(|(fst, lst)| match (fst, lst) {
            (false, false) => Dit,      // Dot .
            (false, true) => Dah,       // Dash -
            (true, false) => Separator, // Letter separator
            (true, true) => Whitespace, // A literal whitespace
        });

        a
    }
}

impl Iterator for U8Iter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter > 7 {
            return None;
        }

        let to_return = (self.val >> (7 - self.counter)) & 1; // ( & 1 ) because better safe than sorry
                                                              // can't rely on transmute

        self.counter += 1;

        Some(unsafe { std::mem::transmute::<u8, bool>(to_return) })
    }
}

impl Clone for U8Iter {
    fn clone(&self) -> Self {
        U8Iter::new(self.val)
    }
}

#[derive(Debug)]
pub enum Morse {
    /// A "dot" -> •
    Dit,
    /// A "dash" -> –
    Dah,
    /// Separates between letters
    Separator,
    /// Separates between words
    Whitespace,
}

impl Display for Morse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Self::Dit => '.',
            Self::Dah => '-',
            Self::Separator => ' ',
            Self::Whitespace => '/',
        })
    }
}
