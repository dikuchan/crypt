pub fn encrypt(string: &str) -> Option<String> {
    let mut result = String::new();

    for letter in string.chars() {
        if letter.is_ascii_alphanumeric() {
            if letter.is_ascii_digit() {
                result.push(letter)
            } else {
                if letter.is_ascii_lowercase() {
                    let shifted = 0x7A - letter as u8 + 0x61;
                    result.push(shifted as char);
                } else {
                    let shifted = 0x5A - letter as u8 + 0x41;
                    result.push(shifted as char);
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
