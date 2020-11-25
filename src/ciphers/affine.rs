use crate::{
    error::*,
    ciphers::cipher::*,
    utils::*,
};

/// Affine cipher is a type of monoalphabetic substitution cipher,
/// where each letter in an alphabet is mapped to its numeric equivalent,
/// encrypted using a simple mathematical function, and converted back to a letter.
///
/// # Algorithm
/// * `A` and `B` - key.
/// * `N` - alphabet size.
/// * `P` - arbitrary letter.
/// * `C` - ciphered letter: `C` = `AP` + `B` (mod `N`).
///
/// `A` should be relatively prime to `N`.
/// The decryption function is `P` = `A`^(-1) (`C` - `B`) (mod `N`).
///
/// # Example
/// Let `A` = 5, `B` = 7, `N` = 26.
/// Then `C` = (5 * `P` + 7) (mod 26), `P` = 21 * (`C` - 7) (mod 26).
///
/// # Requirements
/// All letters should be alpha characters.
pub struct Affine {
    a: i64,
    b: i64,
}

impl Affine {
    pub fn new(a: i64, b: i64) -> Result<Self> {
        let (a, b) = (a % 0x1A, b % 0x1A);
        match gcd(a, 0x1A) {
            1 => Ok(Self { a, b }),
            _ => Err(CipherError::NotPrime(a))
        }
    }
}

impl Cipher for Affine {
    fn encrypt(&self, text: &str) -> Result<String> {
        if !is_alpha(text) { return Err(CipherError::NotAlpha); }

        Ok(text
            .chars()
            .map(|c|
                match c {
                    'a'..='z' => (((self.a * (c as i64 - 0x61) + self.b) % 0x1A) as u8 + 0x61) as char,
                    'A'..='Z' => (((self.a * (c as i64 - 0x41) + self.b) % 0x1A) as u8 + 0x41) as char,
                    _ => c
                })
            .collect())
    }

    fn decrypt(&self, text: &str) -> Result<String> {
        if !is_alpha(text) { return Err(CipherError::NotAlpha); }

        let a = inverse(self.a, 0x1A)?;

        Ok(text.chars()
            .map(|c|
                match c {
                    'a'..='z' => ((((a * (c as i64 - 0x61 - self.b)) % 0x1A + 0x1A) % 0x1A) as u8 + 0x61) as char,
                    'A'..='Z' => ((((a * (c as i64 - 0x41 - self.b)) % 0x1A + 0x1A) % 0x1A) as u8 + 0x41) as char,
                    _ => c
                })
            .collect())
    }
}

#[test]
fn affine_encryption() {
    assert_eq!(Affine::new(5, 8).unwrap().encrypt("Attack at dawn").unwrap(), String::from("Izzisg iz xiov"));
    assert_eq!(Affine::new(15, 9).unwrap().encrypt("true iS None").unwrap(), String::from("iexr zT Wlwr"));
    assert!(Affine::new(14, 2).is_err());
    assert!(Affine::new(25, 37).unwrap().decrypt("Привет, world!").is_err());
}

#[test]
fn affine_decryption() {
    assert_eq!(Affine::new(5, 8).unwrap().decrypt("Izzisg iz xiov").unwrap(),
               String::from("Attack at dawn"));
    assert_eq!(Affine::new(15, 9).unwrap().decrypt("iexr zT Wlwr").unwrap(),
               String::from("true iS None"));
    assert!(Affine::new(14, 2).is_err());
    assert!(Affine::new(25, 37).unwrap().decrypt("Привет, pxuai!").is_err());
}
