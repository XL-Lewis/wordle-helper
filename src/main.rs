use tokio::io;
use tokio::io::AsyncBufReadExt;
use tokio::time::sleep;
use tokio::time::Duration;

use tokio::io::BufReader;

#[tokio::main]

// Components:
// 1. stdin reader
// 2. Alphabet checker
// 3. Common letters checker
// 4. Printer for 2 and 3

async fn main() {
    tokio::spawn(async { read_inputs() });
    loop {}
}

async fn read_inputs() {
    match process_inputs().await {
        Err(e) => {
            println!("Error: {}", e);
        }
        _ => {
            println!("Program ended!")
        }
    }
}

async fn process_inputs() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();

    let mut buffer = BufReader::new(stdin);

    loop {
        let mut line = String::new();
        buffer.read_line(&mut line).await?;
        let word = &line[0..line.len() - 2];

        println!("{}", word.len());
        if word.len() > 5 {
            println!("That's not 5 letters!");
            continue;
        }

        println!("{:?}", line);
        if word == "exit" {
            println!("fail");
            return Ok(());
        }
    }
}
