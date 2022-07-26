use anyhow::{bail, Result};
#[derive(Debug)]
pub struct Guess {
    pub word: String,
    pub double_letters: Vec<char>,
}

impl Guess {
    pub fn new(word: String, max_length: usize) -> Result<Self> {
        if word.len() != max_length {
            bail!("Guess is wrong size!");
        }

        let double_letters = find_double_letters(word.as_ref());
        let ret = Self { word, double_letters };

        Ok(ret)
    }
}

pub fn find_double_letters(input: &str) -> Vec<char> {
    let chars: Vec<char> = input.chars().collect();
    let mut ret = Vec::new();

    for i in 0..chars.len() {
        if chars[i + 1..chars.len()].contains(&chars[i]) {
            if !ret.contains(&chars[i]) {
                ret.push(chars[i]);
            }
        }
    }
    ret
}

#[cfg(test)]
mod test_double_letter {
    use super::find_double_letters;

    #[test]
    fn test_no_double_letter() {
        let double_letters = find_double_letters("input");
        assert_eq!(double_letters, [])
    }

    #[test]
    fn test_one_double_letter() {
        let double_letters = find_double_letters("aabcd");
        assert_eq!(double_letters, ['a'])
    }

    #[test]
    fn test_multiple_double_letters() {
        let double_letters = find_double_letters("aabbccdceff");
        println!("{:?}", double_letters);
        assert!(double_letters.contains(&'a') && double_letters.contains(&'b') && double_letters.contains(&'c') && double_letters.contains(&'f'))
    }
}
