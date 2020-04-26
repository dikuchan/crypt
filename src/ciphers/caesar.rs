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
