use anyhow::{bail, Result};
use indexmap::{indexset, IndexSet};
use std::collections::HashSet;
use std::iter::Iterator;
use std::time::Instant;
use tokio::io;
use tokio::io::AsyncBufReadExt;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

// TODO:
// Track letter placement
// Undo functionality
// reset function
// turn print into a structure or something and push updates to it (live timer?)

use tokio::io::BufReader;

const ALPHABET_LINE_SIZE: usize = 5;
const WORD_LINE_SIZE: usize = 16;

pub struct Letters {
    alph: IndexSet<char>,
    freq: IndexSet<char>,
}

pub struct WordleHelper {
    double_letters: HashSet<char>,
    used_words: Vec<String>,
    letters: Letters,
    timer: Instant,
    word_length: usize,
}

impl WordleHelper {
    fn new(word_length: usize) -> Self {
        Self {
            double_letters: HashSet::new(),
            used_words: Vec::new(),
            letters: Letters::new(),
            timer: Instant::now(),
            word_length,
        }
    }
}

impl Letters {
    fn new() -> Self {
        Self {
            alph: indexset! {'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'},
            freq: indexset! {'e','a','r','i','o','t','n','s','l','c','u','d','p','m','h','g','b','f','y','w','k','v','x','z','j','q'},
        }
    }

    fn remove(&mut self, char: char) -> bool {
        if self.freq.shift_remove(&char) && self.alph.shift_remove(&char) {
            return true;
        }
        return false;
    }
}

#[tokio::main]
async fn main() { tokio::spawn(get_stdin()).await.unwrap(); }

async fn get_stdin() {
    let stdin = io::stdin();
    let mut lines = BufReader::new(stdin).lines();
    let (snd, rcv) = unbounded_channel::<String>();

    // Set word length
    println!("Choose word length . . .");
    loop {
        if let Some(input) = lines.next_line().await.expect("stdin is closed!") {
            if let Ok(word_length) = input.parse::<usize>() {
                println!("Okay! Word length is: {}", word_length);
                tokio::spawn(word_handler(word_length, rcv));
                break;
            } else {
                println!("Word length must be a valid positive integer!")
            }
        }
    }

    // Run main loop
    loop {
        if let Some(word) = lines.next_line().await.expect("stdin is closed!") {
            if word == "q" {
                println!("Exiting . . .");
                break;
            }
            snd.send(word).expect("Failed to send to word checker");
        }
    }
}

async fn word_handler(word_length: usize, mut rcv: UnboundedReceiver<String>) {
    let mut data = WordleHelper::new(word_length);

    loop {
        let word = match check_word(rcv.recv().await.unwrap(), data.word_length) {
            Ok(val) => val,
            Err(e) => {
                print!("{}", e);
                continue;
            },
        };

        let rmvd = data.process_word(word).await;
        data.print_stuffs(rmvd);
    }
}
impl WordleHelper {
    async fn process_word(&mut self, word: String) -> Vec<char> {
        // Check word for double letters
        for letter in check_double_letter(&word) {
            self.double_letters.insert(letter);
        }

        // Remove used letters from arrays
        let mut removed_letters = Vec::new();
        for letter in word.chars() {
            if self.letters.remove(letter) {
                removed_letters.push(letter);
            }
        }

        self.used_words.push(word.to_string());
        removed_letters
    }

    fn print_stuffs(&mut self, rmvd: Vec<char>) {
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
}

fn group_iter_into_blocks<T: ToString>(num_items: usize, data: impl Iterator<Item = T>, buffer: &str) -> Vec<String> {
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

fn check_word(word: String, expected_size: usize) -> Result<String> {
    if !word.is_ascii() {
        bail!("Word [{}] was not valid ascii!", word);
    }
    if word.len() != expected_size {
        bail!("Word [{}] was incorrect size!", word);
    }
    Ok(word)
}

fn check_double_letter(input: &str) -> Vec<char> {
    let chars: Vec<char> = input.chars().collect();
    let mut ret = Vec::new();

    for i in 0..chars.len() {
        let mut j = chars.len() - 1;
        while j != i {
            if chars[i] == chars[j] {
                ret.push(chars[i]);
            }
            j -= 1;
        }
    }
    ret
}
