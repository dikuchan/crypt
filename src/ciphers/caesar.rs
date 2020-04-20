pub fn encrypt(string: &str, shift: u8) -> Option<String> {
    let mut result = String::new();

    if shift > 25 {
        return None;
    } else if shift == 0 {
        return Some(string.to_string());
    }

    for letter in string.chars() {
        if letter.is_ascii_alphabetic() {
            let mut shifted = letter as u8 + shift;
            if letter.is_ascii_lowercase() {
                if shifted > 0x7A {
                    shifted = shifted - 0x7A + 0x61 - 1
                }
            } else {
                if shifted > 0x5A {
                    shifted = shifted - 0x5A + 0x41 - 1
                }
            }
            result.push(shifted as char);
        } else if letter.is_ascii() {
            result.push(letter)
        } else {
            return None;
        }
    }

    Some(result)
}

pub fn decrypt(string: &str, shift: u8) -> Option<String> {
    let mut result = String::new();

    if shift > 25 {
        return None;
    } else if shift == 0 {
        return Some(string.to_string());
    }

    for letter in string.chars() {
        if letter.is_ascii_alphabetic() {
            let mut shifted = letter as u8 - shift;
            if letter.is_ascii_lowercase() {
                if shifted < 0x61 {
                    shifted = shifted + 0x7A - 0x61 + 1
                }
            } else {
                if shifted < 0x41 {
                    shifted = shifted + 0x5A - 0x41 + 1
                }
            }
            result.push(shifted as char);
        } else if letter.is_ascii() {
            result.push(letter)
        } else {
            return None;
        }
    }

    Some(result)
}
