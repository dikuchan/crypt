use crate::{
    error::*,
    ciphers::cipher::*,
};

/// Caesar cipher is one of the simplest ciphers.
///
/// # Algorithm
/// Each letter is replaced by a letter some fixed number of positions down the alphabet.
pub struct Caesar {
    shift: u8,
}

impl Caesar {
    pub fn new(shift: u8) -> Result<Self> {
        match shift {
            0 => Err(CipherError::NullShift),
            _ => Ok(Self { shift: shift % 0x1A })
        }
    }
}

impl Cipher for Caesar {
    fn encrypt(&self, text: &str) -> Result<String> {
        Ok(text.chars()
            .map(|c|
                match c {
                    'a'..='z' => ((c as u8 + self.shift - 0x61) % 0x1A + 0x61) as char,
                    'A'..='Z' => ((c as u8 + self.shift - 0x41) % 0x1A + 0x41) as char,
                    _ => c
                })
            .collect())
    }

    fn decrypt(&self, text: &str) -> Result<String> {
        Ok(text.chars()
            .map(|c|
                match c {
                    'a'..='z' => ((c as u8 + (0x1A - self.shift) - 0x61) % 0x1A + 0x61) as char,
                    'A'..='Z' => ((c as u8 + (0x1A - self.shift) - 0x41) % 0x1A + 0x41) as char,
                    _ => c
                })
            .collect())
    }
}

#[test]
fn caesar_encryption() {
    assert_eq!(Caesar::new(5).unwrap().encrypt("Attack at dawn").unwrap(), String::from("Fyyfhp fy ifbs"));
    assert_eq!(Caesar::new(13).unwrap().encrypt("true iS 42").unwrap(), String::from("gehr vF 42"));
    assert_eq!(Caesar::new(33).unwrap().encrypt("こんばんは, mates").unwrap(), String::from("こんばんは, thalz"));
    assert_eq!(Caesar::new(25).unwrap().encrypt("Привет, world!").unwrap(), String::from("Привет, vnqkc!"));
}

#[test]
fn caesar_decryption() {
    assert_eq!(Caesar::new(5).unwrap().decrypt("Fyyfhp fy ifbs").unwrap(), String::from("Attack at dawn"));
    assert_eq!(Caesar::new(13).unwrap().decrypt("gehr vF 42").unwrap(), String::from("true iS 42"));
    assert_eq!(Caesar::new(33).unwrap().decrypt("こんばんは, thalz").unwrap(), String::from("こんばんは, mates"));
    assert_eq!(Caesar::new(25).unwrap().decrypt("Привет, vnqkc!").unwrap(), String::from("Привет, world!"));
}
