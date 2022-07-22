use anyhow::{bail, Result};

#[derive(Debug, Eq, PartialEq)]
pub enum Commands {
    Exit,
    Reset,
    Placement,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    pub command: Commands,
    pub args: Vec<String>,
}

impl Commands {
    pub fn expected_size(&self) -> usize {
        use Commands::*;

        match self {
            Exit => 1,
            Reset => 2,
            Placement => 2,
        }
    }

    pub fn type_from_str(arg: &str) -> Result<Self> {
        use Commands::*;

        Ok(match arg {
            "-e" => Exit,
            "-r" => Reset,
            "-p" => Placement,
            _ => bail!("Unsupported arg: {}", arg),
        })
    }
}

impl Command {
    pub fn new(inputs: Vec<&str>) -> Result<Self> {
        let command = Commands::type_from_str(inputs[0])?;

        if inputs.len() < command.expected_size() {
            bail!(
                "Argument too short! Expected: {}, Found: {}",
                command.expected_size(),
                inputs.len()
            );
        }

        let args: Vec<String> = inputs.iter().skip(1).map(|&arg| String::from(arg)).collect();
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
            let command = Commands::type_from_str("-p")?;
            assert_eq!(command, Placement);
            Ok(())
        }

        #[test]
        fn test_invalid_arg() -> Result<()> {
            let command = Commands::type_from_str("-z");
            assert!(command.is_err());
            Ok(())
        }

        #[test]
        fn test_expected_size() -> Result<()> {
            let command = Exit;
            assert_eq!(command.expected_size(), 1);
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
