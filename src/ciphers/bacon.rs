use crate::{
    error::*,
    ciphers::cipher::*,
    utils::*,
};

use std::str;

/// Bacon's cipher is a method of message encoding,
/// where message is concealed in the presentation of text, rather than its content.
///
/// # Algorithm
/// Each letter is encoded with a string of five binary digit.
/// The string of encoded characters, called `S`, is zipped with the text.
/// When `S` has `1` in it, a corresponding letter in the zipped text is turned to upper case.
///
/// # Requirements
/// All letters should be alpha characters.
pub struct Bacon<'a> {
    key: &'a str,
}

impl<'a> Bacon<'a> {
    pub fn new(key: &'a str) -> Self {
        Self { key }
    }
}

impl<'a> Cipher for Bacon<'a> {
    fn encrypt(&self, text: &str) -> Result<String> {
        if !is_alpha(text) { return Err(CipherError::NotAlpha); }

        let encrypted = self.key.chars()
            .map(|c|
                format!("{:05b}", match c {
                    'a'..='i' => (c as u8) - 0x61,
                    'j'..='u' => (c as u8) - 0x61 - 0x01,
                    'v'..='z' => (c as u8) - 0x61 - 0x02,
                    'A'..='I' => (c as u8) - 0x41,
                    'J'..='U' => (c as u8) - 0x41 - 0x01,
                    'V'..='Z' => (c as u8) - 0x41 - 0x02,
                    _ => unreachable!()
                }))
            .collect::<String>();

        let result = encrypted.chars()
            .cycle().zip(text.chars())
            .map(|(i, j)| match i {
                '1' => j.to_ascii_uppercase(),
                _ => j
            })
            .collect::<String>();

        Ok(result)
    }

    fn decrypt(&self, text: &str) -> Result<String> {
        Ok(text.chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| if c.is_uppercase() { '1' } else { '0' })
            .collect::<Vec<char>>()
            .chunks(5)
            .map(|chunk| chunk.iter().collect::<String>())
            .map(|string| u8::from_str_radix(&string, 2).unwrap() as char)
            .collect())
    }
}

#[test]
fn bacon_encryption() {
    assert_eq!(Bacon::new("hi").encrypt("Attack at dawn").unwrap(), String::from("AtTACk at daWN"));
    assert_eq!(Bacon::new("hi").encrypt("true is none").unwrap(), String::from("trUE iS none"));
    assert!(Bacon::new("hi").encrypt("こんばんは, mates").is_err());
    assert!(Bacon::new("hi").encrypt("Привет, world!").is_err());
}
