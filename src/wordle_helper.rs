use crate::guess::Guess;
use crate::letters::Letters;
use crate::ALPHABET;
use crate::ALPHABET_LINE_SIZE;
use crate::WORD_LINE_SIZE;

use anyhow::{bail, Result};
use bv::BitVec;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq)]
pub enum InputType {
    WordGuess(String),
    Command(Commands),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Commands {
    Exit,
    Reset,
    GetPlacement(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WordleHelper {
    double_letters: HashSet<char>,
    used_words: Vec<String>,
    letters: Letters,
    letter_placement: HashMap<char, BitVec>,
    timer: Instant,
    word_length: usize,
}

impl WordleHelper {
    pub fn new(word_length: usize) -> Self {
        Self {
            double_letters: HashSet::new(),
            used_words: Vec::new(),
            letters: Letters::new(),
            letter_placement: ALPHABET.iter().map(|letter| (letter.clone(), BitVec::new_fill(true, 5))).collect(),
            timer: Instant::now(),
            word_length,
        }
    }

    pub fn clear(&mut self) {
        self.double_letters = HashSet::new();
        self.used_words = Vec::new();
        self.letters = Letters::new();
        self.letter_placement = ALPHABET.iter().map(|letter| (letter.clone(), BitVec::new_fill(true, 5))).collect();
        self.timer = Instant::now();
    }

    // Remove used letters from arrays and add letter placement
    pub async fn process_word(&mut self, guess: Guess) -> Result<Vec<char>> {
        let mut removed_letters = Vec::new();
        for (index, letter) in guess.word.chars().enumerate() {
            self.letter_placement.get_mut(&letter).unwrap().set(index as u64, false);
            if self.letters.remove(letter).is_some() {
                removed_letters.push(letter);
            }
        }

        guess.double_letters.iter().for_each(|&letter| {
            self.double_letters.insert(letter);
        });

        self.used_words.push(guess.word.to_string());
        Ok(removed_letters)
    }

    pub fn print_stuffs(&mut self, rmvd: Vec<char>) {
        // Format alphabet letters for printing
        let freq_lines = group_iter_into_blocks(ALPHABET_LINE_SIZE, self.letters.freq.iter(), "");
        let alph_lines = group_iter_into_blocks(ALPHABET_LINE_SIZE, self.letters.alph.iter(), "");
        let words = group_iter_into_blocks(WORD_LINE_SIZE, self.used_words.iter(), ", ");

        // Clear terminal
        print!("\x1B[2J\x1B[1;1H");

        // Print Double letters and letters removed from most recent guess
        print!("Removed letters : ");
        rmvd.iter().for_each(|letter| print!("'{}' ", letter));
        print!("\nDouble letters  : ");
        self.double_letters.iter().for_each(|letter| print!("'{}' ", letter));

        // Print used words
        print!("\nWords used      : ");
        words.iter().for_each(|line| println!("{:1$}", line, self.word_length * (WORD_LINE_SIZE + 2)));

        // Print unused letters
        println!("\n----  Unused Letters ----");
        for i in 0..freq_lines.len() {
            println!("|   {:2$}   |   {:2$}   |", alph_lines[i], freq_lines[i], ALPHABET_LINE_SIZE);
        }

        println!("-------------------------");
        println!("Timer: {:?}", self.timer.elapsed());
        println!("\nEnter word: ");
    }

    pub fn get_possible_letter_placement(&self, letter: char) -> String {
        return match self.letter_placement.get(&letter) {
            Some(val) => {
                let mut ret = String::new();
                //TODO: this should be word max length
                for i in 0..val.len() {
                    if val.get(i) {
                        ret.push(letter)
                    } else {
                        ret.push('_')
                    }
                }
                ret
            },
            None => "".to_string(),
        };
    }
}

pub fn group_iter_into_blocks<T: ToString>(num_items: usize, data: impl Iterator<Item = T>, buffer: &str) -> Vec<String> {
    let mut iter = data.peekable();
    let mut ret: Vec<String> = Vec::new();

    while iter.peek().is_some() {
        let mut line = String::new();
        for _ in 0..num_items {
            if let Some(item) = iter.next() {
                line.push_str(&format!("{}", item.to_string()).to_uppercase());
            }
            if iter.peek().is_some() {
                line.push_str(buffer);
            }
        }
        ret.push(line);
    }
    ret
}

pub fn parse_input(word: String) -> Result<InputType> {
    use Commands::*;
    use InputType::*;
    if !word.is_ascii() {
        bail!("Input was invalid Ascii!")
    }
    if word.starts_with("-") {
        let args: Vec<&str> = word.strip_prefix("-").unwrap().split(' ').collect();
        let arg = args[0];
        match args.len() {
            1 => match arg {
                "exit" => return Ok(Command(Exit)),
                "reset" => return Ok(Command(Reset)),
                _ => bail!("Argument not supported!"),
            },
            _ => match arg {
                "l" => return Ok(Command(GetPlacement(args[1].to_string()))),
                _ => bail!("Argument not supported!"),
            },
        }
    }

    Ok(WordGuess(word))
}

#[cfg(test)]
mod test_check_word {
    use super::parse_input;
    use super::Commands::*;
    use super::WordType::*;

    #[test]
    fn test_valid_command() {
        let word = "-l abcd".to_string();
        let res = parse_input(word);

        assert_eq!(res.unwrap(), Command(GetPlacement("abcd".to_string())))
    }

    #[test]
    fn test_invalid_command() {
        let word = "-z".to_string();
        let res = parse_input(word);

        assert!(res.is_err())
    }

    #[test]
    fn test_valid_word() {
        let word = "abcde".to_string();
        let res = parse_input(word);

        assert_eq!(res.unwrap(), Word("abcde".to_string()))
    }
    #[test]
    fn test_invalid_word() {
        let word = "ÆÇ".to_string();
        let res = parse_input(word);

        assert!(res.is_err())
    }
}