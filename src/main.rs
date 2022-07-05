use indexmap::{indexset, IndexSet};
use std::collections::HashSet;
use std::iter::Iterator;
use tokio::io;
use tokio::io::AsyncBufReadExt;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

// TODO:
// Track letter placement
// Undo functionality

use tokio::io::BufReader;

const ALPHABET_LINE_SIZE: usize = 5;
const WORD_LINE_SIZE: usize = 5;
const WORD_LENGTH: usize = 5;

pub struct Letters {
    alph: IndexSet<char>,
    freq: IndexSet<char>,
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
async fn main() {
    let (stdin_snd, stdin_rcv) = unbounded_channel::<String>();
    println!("Starting...");
    tokio::spawn(get_stdin(stdin_snd));
    tokio::spawn(check_word(stdin_rcv));

    loop {}
}

async fn get_stdin(snd: UnboundedSender<String>) {
    let stdin = io::stdin();
    let mut lines = BufReader::new(stdin).lines();

    loop {
        if let Some(word) = lines.next_line().await.unwrap() {
            if word == "q" {
                println!("Exiting . . .");
                break;
            }
            snd.send(word).expect("Failed to send to word checker");
        }
    }
}

async fn check_word(mut rcv: UnboundedReceiver<String>) {
    let mut double_letters = HashSet::new();
    let mut used_words: Vec<String> = Vec::new();
    let mut letters = Letters::new();

    loop {
        print_stuffs(&letters, &used_words);

        // Grab input from stdin
        let word = if let Some(wrd) = rcv.recv().await {
            wrd
        } else {
            break;
        };
        print!("\x1B[2J\x1B[1;1H");

        // Check word validity and
        if word.len() != WORD_LENGTH || !word.is_ascii() {
            println!("Invalid word! It should be 5 ascii letters. Length was {}", word.len());
            continue;
        }
        used_words.push(word.to_string());

        // Double letter check
        for letter in check_double_letter(&word) {
            double_letters.insert(letter);
        }

        // Remove used letters from arrays
        let mut removed_letters = String::new();
        for letter in word.chars() {
            if letters.remove(letter) {
                removed_letters.push(letter);
            }
        }

        // Print double letters used
        print!("\n\nDouble letters:  ");
        println!("{:20?}", double_letters);
        // Print letters removed after previous word
        println!("Removed letters: [{}]", removed_letters);
    }
}

fn print_stuffs(letters: &Letters, used_words: &Vec<String>) {
    // Format previously used words for printing
    let words = group_iter_into_blocks(WORD_LINE_SIZE, used_words.iter(), ", ");

    // Print words used so far
    println!("Words used: ");
    for line in words {
        println!("|{:1$}|", line, WORD_LENGTH);
    }

    // Format letters for printing
    let freq_lines = group_iter_into_blocks(ALPHABET_LINE_SIZE, letters.freq.iter(), "");
    let alph_lines = group_iter_into_blocks(ALPHABET_LINE_SIZE, letters.alph.iter(), "");
    // Print letters left
    println!("\n\nUnused Letters: ");

    for i in 0..freq_lines.len() {
        println!("|   {:2$}   |   {:2$}   |", alph_lines[i], freq_lines[i], ALPHABET_LINE_SIZE);
    }
    println!("-------------------------");

    println!("\nEnter word: ");
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
