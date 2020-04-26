pub fn encrypt(string: &str) -> String {
    string.chars()
        .map(move |c|
            match c {
                'a'..='z' => format!("{:05b}", ((c as u8) - 0x61)),
                'A'..='Z' => format!("{:05b}", ((c as u8) - 0x41)),
                ' ' => String::new(),
                _ => c.to_string(),
            })
        .collect()
}

#[test]
fn test_baconian_encryption() {
    assert_eq!(encrypt("Attack at dawn"), String::from("000001001110011000000001001010000001001100011000001011001101"));
    assert_eq!(encrypt("true iS 42"), String::from("10011100011010000100010001001042"));
    assert_eq!(encrypt("こんばんは, mates"), String::from("こんばんは,0110000000100110010010010"));
    assert_eq!(encrypt("Привет, world!"), String::from("Привет,1011001110100010101100011!"))
}