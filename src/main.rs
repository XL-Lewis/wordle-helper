use core::num;

use tokio::io;
use tokio::io::AsyncBufReadExt;
use tokio::time::sleep;
use tokio::time::Duration;

use tokio::io::BufReader;

const ALPHABET_LINE_SIZE: usize = 4;
const WORD_LINE_SIZE: usize = 3;

#[tokio::main]

// Components:
// 1. stdin reader
// 2. Alphabet checker
// 3. Common letters checker
// 4. Printer for 2 and 3

async fn main() {
    println!("Starting...");
    tokio::spawn(async { process_inputs() }).await.expect("").await.expect("");
}

// async fn read_inputs() {
//     match process_inputs().await {
//         Err(e) => {
//             println!("Error: {}", e);
//         },
//         _ => {
//             println!("Program ended!")
//         },
//     }
// }

async fn process_inputs() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut letters_alphabetical = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
        'y', 'z',
    ];

    let mut letters_frequency = vec![
        'e', 'a', 'r', 'i', 'o', 't', 'n', 's', 'l', 'c', 'u', 'd', 'p', 'm', 'h', 'g', 'b', 'f', 'y', 'w', 'k', 'v', 'x', 'z',
        'j', 'q',
    ];

    let mut used_words: Vec<String> = Vec::new();

    let mut buffer = BufReader::new(stdin);

    loop {
        println!("-----------------");
        let mut line = String::new();
        buffer.read_line(&mut line).await?;
        let word = &line[0..line.len() - 1];

        if word == "q" {
            println!("Exiting . . .");
            return Ok(());
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
        print!("\n\n\n");

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
