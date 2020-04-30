/*
 *  [General]
 *
 *  The Caesar cipher is one of the earliest known and simplest ciphers.
 *  It is a type of substitution cipher in which each letter is 'shifted' a certain number of places down the alphabet.
 *  For example, with a shift of 1, A would be replaced by B, B would become C, and so on.
 *  The method is named after Julius Caesar, who apparently used it to communicate with his generals.
 *  The Caesar cipher offers essentially no communication security, and it can be easily broken even by hand.
 *
 *  [Algorithm]
 *
 *  To pass an encrypted message, it is first necessary that both parties have the 'key' for the cipher.
 *  For the caesar cipher, the key is the number of characters to shift the cipher alphabet.
 *  First we translate all of our characters to numbers, 'a' = 0, 'b' = 1, 'c' = 2, ... , 'z' = 25.
 *  We can now represent the caesar cipher encryption function, e(x), where x is the character we are encrypting, as
 *
 *  e(x) = (x + k) (mod 26)
 *
 *  Where k is the key (the shift) applied to each letter.
 *  After applying this function the result is a number which must then be translated back into a letter.
 *  The decryption function is
 *
 *  e(x) = (x - k) (mod 26)
 *
 *  [Example]
 *
 *  Here is a quick example of the encryption and decryption steps involved with the caesar cipher.
 *  The text we will encrypt is 'defend the east wall of the castle', with a shift of 1.
 *
 *  defend the east wall of the castle
 *  efgfoe uif fbtu xbmm pg uif dbtumf
 *
 *  It is easy to see how each character in the plaintext is shifted up the alphabet.
 *  Decryption is just as easy, by using an offset of -1.
 */

// Source: http://practicalcryptography.com

use crate::utils;

pub fn encrypt(string: &str, shift: u8) -> Result<String, utils::CipherError> {
    let shift = if shift != 0x00 {
        shift % 0x1A
    } else {
        return Err(utils::CipherError::NullShift);
    };

    let result = string.chars()
        .map(move |c|
            match c {
                'a'..='z' => ((c as u8 + shift - 0x61) % 0x1A + 0x61) as char,
                'A'..='Z' => ((c as u8 + shift - 0x41) % 0x1A + 0x41) as char,
                _ => c
            })
        .collect();

    Ok(result)
}

pub fn decrypt(string: &str, shift: u8) -> Result<String, utils::CipherError> {
    let shift = if shift != 0x00 {
        shift % 0x1A
    } else {
        return Err(utils::CipherError::NullShift);
    };

    let result = string.chars()
        .map(move |c|
            match c {
                'a'..='z' => ((c as u8 + (0x1A - shift) - 0x61) % 0x1A + 0x61) as char,
                'A'..='Z' => ((c as u8 + (0x1A - shift) - 0x41) % 0x1A + 0x41) as char,
                _ => c
            })
        .map(|b| b as char)
        .collect();

    Ok(result)
}

#[test]
fn test_caesar_encryption() {
    assert_eq!(encrypt("Attack at dawn", 5).unwrap(), String::from("Fyyfhp fy ifbs"));
    assert_eq!(encrypt("true iS 42", 13).unwrap(), String::from("gehr vF 42"));
    assert_eq!(encrypt("こんばんは, mates", 33).unwrap(), String::from("こんばんは, thalz"));
    assert_eq!(encrypt("Привет, world!", 25).unwrap(), String::from("Привет, vnqkc!"));
}

#[test]
fn test_caesar_decryption() {
    assert_eq!(decrypt("Fyyfhp fy ifbs", 5).unwrap(), String::from("Attack at dawn"));
    assert_eq!(decrypt("gehr vF 42", 13).unwrap(), String::from("true iS 42"));
    assert_eq!(decrypt("こんばんは, thalz", 33).unwrap(), String::from("こんばんは, mates"));
    assert_eq!(decrypt("Привет, vnqkc!", 25).unwrap(), String::from("Привет, world!"));
}
