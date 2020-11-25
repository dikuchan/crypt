use crate::{
    error::*,
    ciphers::cipher::*,
};

/// Atbash is a simple monoalphabetic substitution cipher.
///
/// # Algorithm
/// Each `i`-th letter is replaced with the `i`-th letter from the reversed alphabet.
/// And vice versa in the decoding procedure.
pub struct Atbash;

impl Atbash {
    pub fn new() -> Self { Atbash }
}

impl Cipher for Atbash {
    fn encrypt(&self, text: &str) -> Result<String> {
        Ok(text.chars()
            .map(|c|
                match c {
                    'a'..='z' => (0x7A - c as u8 + 0x61) as char, // 'z' - c + 'a'
                    'A'..='Z' => (0x5A - c as u8 + 0x41) as char, // 'Z' - c + 'A'
                    _ => c
                })
            .collect())
    }
}

#[test]
fn atbash_encryption() {
    assert_eq!(Atbash::new().encrypt("Attack at dawn").unwrap(), String::from("Zggzxp zg wzdm"));
    assert_eq!(Atbash::new().encrypt("true iS 42").unwrap(), String::from("gifv rH 42"));
    assert_eq!(Atbash::new().encrypt("こんばんは").unwrap(), String::from("こんばんは"));
    assert_eq!(Atbash::new().encrypt("Привет, world!").unwrap(), String::from("Привет, dliow!"))
}

#[test]
fn atbash_decryption() {
    assert_eq!(Atbash::new().decrypt("Zggzxp zg wzdm").unwrap(), String::from("Attack at dawn"));
    assert_eq!(Atbash::new().decrypt("gifv rH 42").unwrap(), String::from("true iS 42"));
    assert_eq!(Atbash::new().decrypt("こんばんは").unwrap(), String::from("こんばんは"));
    assert_eq!(Atbash::new().decrypt("Привет, dliow!").unwrap(), String::from("Привет, world!"))
}
