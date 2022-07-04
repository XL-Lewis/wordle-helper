use std::collections::HashSet;
use tokio::io;
use tokio::io::AsyncBufReadExt;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

// TODO:
// Track letter placement
// Improve efficiency + data structures (hashmaps?)
// Undo functionality

use tokio::io::BufReader;

const ALPHABET_LINE_SIZE: usize = 5;
const WORD_LINE_SIZE: usize = 5;

// Components:
// 1. stdin reader
// 2. Alphabet checker
// 3. Common letters checker
// 4. Printer for 2 and 3

// Alphabet struct:
// hashmap with
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
            snd.send(word).expect("Failed to send to word checker");
        }
    }
}

async fn check_word(mut rcv: UnboundedReceiver<String>) {
    let mut letters_alphabetical = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
        'y', 'z',
    ];
    let mut letters_frequency = vec![
        'e', 'a', 'r', 'i', 'o', 't', 'n', 's', 'l', 'c', 'u', 'd', 'p', 'm', 'h', 'g', 'b', 'f', 'y', 'w', 'k', 'v', 'x', 'z',
        'j', 'q',
    ];
    let mut double_letters = HashSet::new();
    let mut used_words: Vec<String> = Vec::new();

    loop {
        println!("-----------------\n\n");
        println!("Enter word: ");

        let word = if let Some(wrd) = rcv.recv().await {
            wrd
        } else {
            break;
        };

        for letter in check_double_letter(&word) {
            double_letters.insert(letter);
        }

        if word == "q" {
            println!("Exiting . . .");
            break;
        }

        // check length and store word as used
        if word.len() != 5 || !word.is_ascii() {
            println!("Invalid word! It should be 5 ascii letters. Length was {}", word.len());
            continue;
        }

        used_words.push(word.to_string());

        // Remove used letters from arrays
        for letter in word.chars() {
            for i in 0..letters_alphabetical.len() - 1 {
                let stored_letter = letters_alphabetical[i];
                if letter == stored_letter {
                    letters_alphabetical.remove(i);
                    for j in 0..letters_frequency.len() - 1 {
                        let stored_letter = letters_frequency[j];
                        if letter == stored_letter {
                            letters_frequency.remove(j);
                        }
                    }
                    continue;
                }
            }
        }

        println!("\n\nLetters left: ");

        let mut alph_index = 0;

        // Print all letters left
        while alph_index < letters_alphabetical.len() {
            let a = get_next_x_items_from_array(ALPHABET_LINE_SIZE, alph_index, &letters_alphabetical, "");
            let b = get_next_x_items_from_array(ALPHABET_LINE_SIZE, alph_index, &letters_frequency, "");
            alph_index += ALPHABET_LINE_SIZE;

            println!("{}      {}", a.to_uppercase(), b.to_uppercase());
        }

        println!("\n");
        print!("Double letters: ");
        for letter in &double_letters {
            print!("{}", letter)
        }

        println!("\n\n");

        // Print all used words
        let mut word_index = 0;
        while word_index < used_words.len() {
            let word_line = get_next_x_items_from_array(WORD_LINE_SIZE, word_index, &used_words, ", ");
            println!("{}", word_line.to_uppercase());
            word_index += WORD_LINE_SIZE;
        }
    }
}

fn get_next_x_items_from_array<T: ToString>(num_items: usize, index: usize, arr: &Vec<T>, buffer: &str) -> String {
    let mut ret: String = "".to_string();
    for i in index..index + num_items {
        let maybe = match &arr.get(i) {
            Some(val) => val.to_string() + &buffer.to_string(),
            None => " ".to_string(),
        };
        ret.push_str(&maybe);
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
