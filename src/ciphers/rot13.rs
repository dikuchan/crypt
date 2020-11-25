use crate::{
    error::*,
    ciphers::cipher::*,
};

/// # Algorithm
/// Essentially, the Caesar cipher with the fixed shift of half of the alphabet.
/// In particular, shift equals `13` when using the Latin alphabet.
pub struct Rot13;

impl Rot13 {
    pub fn new() -> Self { Rot13 }
}

impl Cipher for Rot13 {
    fn encrypt(&self, text: &str) -> Result<String> {
        Ok(text.chars()
            .map(|c|
                match c {
                    'a'..='m' => (c as u8 + 0x0D) as char, // c + 13
                    'n'..='z' => (c as u8 - 0x0D) as char, // c - 13
                    'A'..='M' => (c as u8 + 0x0D) as char, // c + 13
                    'N'..='Z' => (c as u8 - 0x0D) as char, // c - 13
                    _ => c
                })
            .collect())
    }
}

#[test]
fn rot13_encryption() {
    assert_eq!(Rot13::new().encrypt("Attack at dawn").unwrap(), String::from("Nggnpx ng qnja"));
    assert_eq!(Rot13::new().encrypt("true iS 42").unwrap(), String::from("gehr vF 42"));
    assert_eq!(Rot13::new().encrypt("こんばんは").unwrap(), String::from("こんばんは"));
    assert_eq!(Rot13::new().encrypt("Привет, world!").unwrap(), String::from("Привет, jbeyq!"));
}

#[test]
fn rot13_decryption() {
    assert_eq!(Rot13::new().decrypt("Nggnpx ng qnja").unwrap(), String::from("Attack at dawn"));
    assert_eq!(Rot13::new().decrypt("gehr vF 42").unwrap(), String::from("true iS 42"));
    assert_eq!(Rot13::new().decrypt("こんばんは").unwrap(), String::from("こんばんは"));
    assert_eq!(Rot13::new().decrypt("Привет, jbeyq!").unwrap(), String::from("Привет, world!"));
}
