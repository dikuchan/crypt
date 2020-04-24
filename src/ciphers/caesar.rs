pub fn encode(string: &str, shift: u8) -> String {
    let shift = if shift != 0x00 {
        shift % 0x1A
    } else {
        return string.to_string();
    };

    string.chars()
        .map(move |c|
            match c {
                'a'..='z' => ((c as u8 + shift - 0x61) % 0x1A + 0x61) as char,
                'A'..='Z' => ((c as u8 + shift - 0x41) % 0x1A + 0x41) as char,
                _ => c
            })
        .collect()
}

pub fn decode(string: &str, shift: u8) -> String {
    let shift = if shift != 0x00 {
        shift % 0x1A
    } else {
        return string.to_string();
    };

    string.chars()
        .map(move |c|
            match c {
                'a'..='z' => ((c as u8 + (0x1A - shift) - 0x61) % 0x1A + 0x61) as char,
                'A'..='Z' => ((c as u8 + (0x1A - shift) - 0x41) % 0x1A + 0x41) as char,
                _ => c
            })
        .map(|b| b as char)
        .collect()
}

#[test]
fn test_caesar_encryption() {
    assert_eq!(encode("Attack at dawn", 5), String::from("Fyyfhp fy ifbs"));
    assert_eq!(encode("true iS 42", 13), String::from("gehr vF 42"));
    assert_eq!(encode("こんばんは, mates", 33), String::from("こんばんは, thalz"));
    assert_eq!(encode("Привет, world!", 25), String::from("Привет, vnqkc!"));
}

#[test]
fn test_caesar_decryption() {
    assert_eq!(decode("Fyyfhp fy ifbs", 5), String::from("Attack at dawn"));
    assert_eq!(decode("gehr vF 42", 13), String::from("true iS 42"));
    assert_eq!(decode("こんばんは, thalz", 33), String::from("こんばんは, mates"));
    assert_eq!(decode("Привет, vnqkc!", 25), String::from("Привет, world!"));
}