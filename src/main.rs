mod args;
mod guess;
mod letters;
mod wordle_helper;

use args::Commands;
use guess::Guess;
use tokio::io;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use wordle_helper::WordleHelper;
use wordle_helper::*;

static ALPHABET_LINE_SIZE: usize = 5;
static WORD_LINE_SIZE: usize = 16;
const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
    'z',
];
const ALPHABET_BY_FREQUENCY: [char; 26] = [
    'e', 'a', 'r', 'i', 'o', 't', 'n', 's', 'l', 'c', 'u', 'd', 'p', 'm', 'h', 'g', 'b', 'f', 'y', 'w', 'k', 'v', 'x', 'z', 'j',
    'q',
];

// TODO:
// Undo functionality
// turn print into a structure or something and push updates to it (live timer?)

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
            snd.send(word).expect("Failed to send to word checker");
        }
    }
}

async fn word_handler(word_length: usize, mut rcv: UnboundedReceiver<String>) {
    use Commands::*;
    use InputType::*;
    let mut data = WordleHelper::new(word_length.clone());
    loop {
        match parse_input(rcv.recv().await.unwrap()) {
            Ok(WordGuess(guess)) => match Guess::new(guess, word_length) {
                Ok(guess) => {
                    let rmvd = data.process_word(guess).await.unwrap();
                    data.print_stuffs(rmvd);
                },
                Err(e) => {
                    print!("{:?}", e);
                    continue;
                },
            },
            Ok(Command(cmd)) => match cmd.command {
                Exit => break,
                Reset => {
                    data.clear();
                    continue;
                },
                Placement => {
                    let unknown_letters = &cmd.args[0];
                    let known_letters = match cmd.args.get(1) {
                        None => (0..word_length).map(|_| "_").collect(),
                        Some(letters) => letters.to_string(),
                    };

                    println!("Showing placement for letters: {}", unknown_letters);
                    // For each letter in arg
                    for char in unknown_letters.chars() {
                        // Get possible placements for letter
                        let positions = data.get_possible_letter_placement(char);
                        // If we have known letters in our word, replace with those instead
                        let print_str = known_letters
                            .chars()
                            .zip(positions.chars())
                            .map(|(known, position)| match known {
                                '_' => position.to_ascii_lowercase(),
                                _ => known.to_ascii_uppercase(),
                            })
                            .collect::<String>();

                        println!("{}", print_str);
                    }
                },
            },

            Err(e) => {
                println!("{}", e);
                continue;
            },
        };
    }
}
