use std::collections::{HashMap, HashSet};
use std::time::Instant;

use anyhow::{bail, Result};
use bv::BitVec;

use crate::args::Command;
use crate::guess::Guess;
use crate::letters::Letters;
use crate::{ALPHABET, ALPHABET_LINE_SIZE};

#[derive(Debug, Eq, PartialEq)]
pub enum InputType {
    WordGuess(String),
    Command(Command),
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
            letter_placement: ALPHABET.iter().map(|letter| (letter.clone(), BitVec::new_fill(true, word_length as u64))).collect(),
            timer: Instant::now(),
            word_length,
        }
    }

    pub fn clear(&mut self) {
        self.double_letters = HashSet::new();
        self.used_words = Vec::new();
        self.letters = Letters::new();
        self.letter_placement = ALPHABET.iter().map(|letter| (letter.clone(), BitVec::new_fill(true, self.word_length as u64))).collect();
        self.timer = Instant::now();
    }

    // Remove used letters from arrays and add letter placement
    pub async fn process_word(&mut self, guess: Guess) -> Result<Vec<char>> {
        let mut removed_letters = Vec::new();

        for (index, letter) in guess.word.chars().enumerate() {
            // Letter placement
            self.letter_placement.get_mut(&letter).unwrap().set(index as u64, false);
            // Removed letters
            if self.letters.remove(letter).is_some() {
                removed_letters.push(letter);
            }
        }

        // Add double letters
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
        //let words = group_iter_into_blocks(WORD_LINE_SIZE, self.used_words.iter(), ", ");

        // Clear terminal
        print!("\x1B[2J\x1B[1;1H");
        println!("Timer           : {:.2?}", self.timer.elapsed());
        // Print Double letters and letters removed from most recent guess
        print!("Recent Removals : ");
        rmvd.iter().for_each(|letter| print!("'{}' ", letter));
        print!("\nDoubles         : ");
        self.double_letters.iter().for_each(|letter| print!("'{}' ", letter));

        // Print unused letters
        println!("\n----  Unused Letters ----");
        for i in 0..freq_lines.len() {
            println!("|   {:2$}   |   {:2$}   |", alph_lines[i], freq_lines[i], ALPHABET_LINE_SIZE);
        }

        println!("-------------------------\n");
        println!("-------------------------");

        println!("Input word or command: ");
    }

    pub fn get_possible_letter_placement(&self, letter: char) -> String {
        return match self.letter_placement.get(&letter) {
            Some(val) => {
                let mut ret = String::new();
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

    pub fn print_possible_letter_placement(&self, args: Vec<String>) {
        println!("");
        let unknown_letters = &args[0];
        let known_letters = match args.get(1) {
            None => (0..self.word_length).map(|_| "_").collect(),
            Some(letters) => {
                println!("{}", letters.to_ascii_uppercase());
                letters.to_string()
            },
        };

        // For each letter in arg
        for char in unknown_letters.chars() {
            // Get possible placements for letter
            let positions = self.get_possible_letter_placement(char);
            // If we have known letters in our word, replace with those instead
            let print_str = known_letters
                .chars()
                .zip(positions.chars())
                .map(|(known, position)| match known {
                    '_' => position.to_ascii_lowercase(),
                    _ => '_',
                })
                .collect::<String>();

            println!("{}", print_str);
        }
        println!("");
    }

    pub fn show_possible_combinations(&self, args: Vec<String>) {
        println!("");
        let unknown_letters = &args[0];
        let known_letters = match args.get(1) {
            None => (0..self.word_length).map(|_| "_").collect(),
            Some(letters) => {
                println!("{}\n", letters.to_ascii_uppercase());
                letters.to_string()
            },
        };
        let mut guesses: Vec<String> = vec![known_letters];

        for letter in unknown_letters.chars() {
            let mut updated_guesses: Vec<String> = Vec::new();
            for guess in &guesses {
                // TODO: This could use a rewrite
                let placements = self.get_possible_letter_placement(letter);
                let mut expanded = get_possible_placement_strings(letter, &placements, guess).to_owned();
                updated_guesses.extend_from_slice(&mut expanded)
            }
            if !updated_guesses.is_empty() {
                guesses = updated_guesses.to_vec();
            }
        }

        for guess in guesses {
            println!("{}", guess);
        }
        println!("");
    }
}

// Takes a letter ('y') and a guess ("A_B_C") and returns every combo of that letter in that string ("AYBC", "A_BYC")
pub fn get_possible_placement_strings(letter: char, potential_placements: &str, word: &str) -> Vec<String> {
    // TODO: Breaks if you try more letters than there are guesses
    let mut guesses: Vec<String> = Vec::new();
    for (index, value) in word.chars().enumerate() {
        if value == '_' {
            match potential_placements.get(index..index + 1) {
                Some("_") | None => {},
                Some(_) => {
                    let mut guess = word.to_string();
                    guess.replace_range(index..index + 1, &letter.to_uppercase().to_string());
                    guesses.push(guess);
                },
            }
        }
    }

    return guesses;
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

pub fn parse_input(input: String) -> Result<InputType> {
    if !input.is_ascii() {
        bail!("Input was invalid Ascii!")
    }
    if input.starts_with("-") {
        let args: Vec<&str> = input.split_ascii_whitespace().collect();
        let cmd = Command::new(args)?;
        return Ok(InputType::Command(cmd));
    }
    Ok(InputType::WordGuess(input))
}

#[cfg(test)]
mod test_check_word {
    use super::InputType::*;
    use super::{get_possible_placement_strings, parse_input};

    #[test]
    fn test_valid_word() {
        let word = "abcde".to_string();
        let res = parse_input(word);

        assert_eq!(res.unwrap(), WordGuess("abcde".to_string()))
    }
    #[test]
    fn test_invalid_word() {
        let word = "ÆÇ".to_string();
        let res = parse_input(word);

        assert!(res.is_err())
    }
    #[test]
    fn test_get_possible_guesses() {
        let known = "A_E_F";
        let placements = "___ZZ".to_string();
        let res = get_possible_placement_strings('z', &placements, known);

        assert_eq!(res, ["A_EZF"])
    }
}
