/*
 *  [General]
 *
 *  The Baconian cipher is a substitution cipher in which each letter is replaced by a sequence of 5 characters.
 *  This cipher offers very little communication security, as it is a substitution cipher.
 *  As such all the methods used to cryptanalyse substitution ciphers can be used to break Baconian ciphers.
 *  The main advantage of the cipher is that it allows hiding the fact that a secret message has been sent at all.
 *
 *  [Algorithm]
 *
 *  Each letter is assigned to a string of five binary digits.
 *  These could be the letters 'A' and 'B', the numbers 0 and 1 or whatever else you may desire.
 *
 *  [Example]
 *
 *  To encipher a message, e.g. 'STRIKE NOW', we replace each letter:
 *
 *  S     T     R     I     K     E     N     O     W
 *  baaab baaba baaaa abaaa abaab aabaa abbaa abbab babaa
 *  Hold OFf uNtIl you hEar frOm mE agAin. wE May cOMpROmIse.
 *
 *  Capital letters are used where the cipher has a 'b' and lowercase where there is an 'a'.
 *  This scheme is a little bit transparent, however there are many ways of encoding a Baconian cipher in text.
 */

// Source: http://practicalcryptography.com

use crate::utils::*;
use std::str;

pub fn encrypt(string: &str) -> Result<String, CipherError> {
    if !is_alpha(string) {
        return Err(CipherError::NotAlpha);
    }
    let string = string.to_uppercase();
    let result = string.chars()
        .map(|c|
            if c.is_ascii_whitespace() {
                c.to_string()
            } else {
                format!("{:05b}", match c {
                    'A'..='I' => (c as u8) - 0x41,
                    'J'..='U' => (c as u8) - 0x41 - 0x01,
                    'V'..='Z' => (c as u8) - 0x41 - 0x02,
                    _ => unreachable!()
                })
            })
        .collect();

    Ok(result)
}

fn transform(slice: &[u8]) -> Option<char> {
    let string = if let Ok(string) = str::from_utf8(&slice) {
        string
    } else {
        return None;
    };

    let result = if let Ok(result) = u8::from_str_radix(string, 2) {
        match result {
            0x00..=0x08 => result + 0x41,
            0x09..=0x13 => result + 0x41 + 0x01,
            0x14..=0x17 => result + 0x41 + 0x02,
            _ => unreachable!()
        }
    } else {
        return None;
    };

    Some(result as char)
}

pub fn decrypt(string: &str) -> Result<String, CipherError> {
    let chunks = string.split_whitespace().collect::<Vec<&str>>();
    let result = chunks.iter()
        .map(|&chunk|
            chunk.as_bytes()
                .chunks(5)
                .map(|c| transform(c).unwrap())
                .collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");

    Ok(result)
}

#[test]
fn test_baconian_encryption() {
    assert_eq!(encrypt("Attack at dawn").unwrap(), String::from("000001001010010000000001001001 0000010010 00011000001010001100"));
    assert_eq!(encrypt("true iS None").unwrap(), String::from("10010100001001100100 0100010001 01100011010110000100"));
    assert!(encrypt("こんばんは, mates").is_err());
    assert!(encrypt("Привет, world!").is_err())
}

#[test]
fn test_baconian_decryption() {
    assert_eq!(decrypt("000001001010010000000001001001 0000010010 00011000001010001100").unwrap(), String::from("ATTACK AT DAWN"));
    assert_eq!(decrypt("10010100001001100100 0100010001 01100011010110000100").unwrap(), String::from("TRUE IS NONE"));
}