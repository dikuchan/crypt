pub fn encrypt(string: &str) -> Option<String> {
    let mut result = String::new();

    for letter in string.chars() {
        if letter.is_ascii_alphanumeric() {
            if letter.is_ascii_digit() {
                result.push(letter)
            } else {
                if letter.is_ascii_lowercase() {
                    if letter >= 'a' && letter <= 'm' {
                        let shifted = letter as u8 + 0xD;
                        result.push(shifted as char);
                    } else {
                        let shifted = letter as u8 - 0xD;
                        result.push(shifted as char);
                    }
                } else {
                    if letter >= 'A' && letter <= 'M' {
                        let shifted = letter as u8 + 0xD;
                        result.push(shifted as char);
                    } else {
                        let shifted = letter as u8 - 0xD;
                        result.push(shifted as char);
                    }
                }
            }
        } else if letter.is_ascii() {
            result.push(letter)
        } else {
            return None;
        }
    }

    Some(result)
}
