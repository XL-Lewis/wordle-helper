use anyhow::{bail, Result};

#[derive(Debug, Eq, PartialEq)]
pub enum Commands {
    /// Exit Program
    Exit,
    /// Clear screen and reprint available letters
    Clear,
    /// Reset data for new wordle game
    Reset,
    /// Print placement of letters
    Placement,
}

impl Commands {
    pub fn expected_num_args(&self) -> usize {
        use Commands::*;
        match self {
            Exit => 1,
            Clear => 1,
            Reset => 2,
            Placement => 2,
        }
    }

    /// Get the expected number of args for a particular command
    pub fn has_valid_length(&self, size: usize) -> bool { return self.expected_num_args() <= size; }

    /// Convert input string into argument type
    pub fn command_from_str(arg: &str) -> Result<Self> {
        use Commands::*;

        Ok(match arg {
            "-e" => Exit,
            "-c" => Clear,
            "-r" => Reset,
            "-p" => Placement,
            _ => bail!("Unsupported arg: {}", arg),
        })
    }

    pub fn example_usage(&self) -> String {
        use Commands::*;
        match self {
            Exit => "-e",
            Clear => "'-c'",
            Reset => "'-r [word_size]'",
            Placement => "-p [Letters] [Word Layout]\nExample: -p av C_R_E",
        }
        .to_string()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    pub command: Commands,
    pub args: Vec<String>,
}

impl Command {
    /// Convert an input string into the respective command struct
    ///
    /// # Example:
    /// ```text
    /// -p abcd _j_f_
    /// ```
    pub fn new(inputs: Vec<&str>) -> Result<Self> {
        let command = Commands::command_from_str(inputs[0])?;
        let args: Vec<String> = match command.has_valid_length(inputs.len()) {
            true => inputs.iter().skip(1).map(|&arg| String::from(arg)).collect(),
            false => bail!(
                "Expected minimum {} args. Found {} instead.\nUsage: {}",
                command.expected_num_args(),
                inputs.len(),
                command.example_usage()
            ),
        };
        Ok(Self { command, args })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod commands {
        use super::*;
        use Commands::*;
        #[test]
        fn test_valid_arg() -> Result<()> {
            let command = Commands::command_from_str("-p")?;
            assert_eq!(command, Placement);
            Ok(())
        }

        #[test]
        fn test_invalid_arg() -> Result<()> {
            let command = Commands::command_from_str("-z");
            assert!(command.is_err());
            Ok(())
        }

        #[test]
        fn test_valid_expected_size() -> Result<()> {
            let command = Exit;
            assert!(command.has_valid_length(1));
            Ok(())
        }

        #[test]
        fn test_invalid_expected_size() -> Result<()> {
            let command = Placement;
            assert!(!command.has_valid_length(1));
            Ok(())
        }

        #[test]
        fn test_example_usage() -> Result<()> {
            let command = Commands::command_from_str("-p")?;
            println!("{}", command.example_usage());
            Ok(())
        }
    }

    mod command {
        use super::*;

        #[test]
        fn test_new() -> Result<()> {
            let cmd = Command::new(["-p", "abcde", "defg"].to_vec())?;
            assert_eq!(cmd.command, Commands::Placement);
            assert_eq!(cmd.args, ["abcde", "defg"].to_vec());

            Ok(())
        }

        #[test]
        fn test_too_short() -> Result<()> {
            let cmd = Command::new(["-p"].to_vec());
            assert!(cmd.is_err());
            Ok(())
        }
    }
}
