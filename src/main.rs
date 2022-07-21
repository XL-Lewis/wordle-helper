mod guess;
mod letters;
mod wordle_helper;

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
// Track letter placement
// Undo functionality
// reset function
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
            Ok(WordGuess(guess)) => {
                let res = Guess::new(guess, word_length);
                if let Ok(guess) = res {
                    let rmvd = data.process_word(guess).await.unwrap();
                    data.print_stuffs(rmvd);
                } else {
                    print!("{:?}", res.unwrap_err());
                    continue;
                }
            },
            Ok(Command(GetPlacement(letters))) => {
                println!("Showing placement for letters: {}", letters);

                for char in letters.chars() {
                    println!("{}", data.get_possible_letter_placement(char));
                }
            },
            Ok(Command(Exit)) => break,
            Ok(Command(Reset)) => {
                data.clear();
                continue;
            },
            Err(e) => {
                print!("{}", e);
                continue;
            },
        };
    }
}
