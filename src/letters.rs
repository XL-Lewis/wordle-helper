use indexmap::IndexSet;

use crate::{ALPHABET, ALPHABET_BY_FREQUENCY};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Letters {
    pub alph: IndexSet<char>,
    pub freq: IndexSet<char>,
}

impl Letters {
    pub fn new() -> Self {
        let alph: IndexSet<char> = ALPHABET.iter().map(|char| *char).collect();
        let freq: IndexSet<char> = ALPHABET_BY_FREQUENCY.iter().map(|char| *char).collect();
        Self { alph, freq }
    }

    pub fn remove(&mut self, letter: char) -> Option<char> {
        if self.freq.shift_remove(&letter) && self.alph.shift_remove(&letter) {
            return Some(letter);
        }
        return None;
    }
}
