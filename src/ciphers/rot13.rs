pub fn encrypt(string: &str) -> String {
    string.chars()
        .map(|c|
            match c {
                'a'..='m' => (c as u8 + 0x0D) as char, // c + 13
                'n'..='z' => (c as u8 - 0x0D) as char, // c - 13
                'A'..='M' => (c as u8 + 0x0D) as char, // c + 13
                'N'..='Z' => (c as u8 - 0x0D) as char, // c - 13
                _ => c
            })
        .collect()
}

#[test]
fn test_rot13_encryption() {
    assert_eq!(encrypt("Attack at dawn"), String::from("Nggnpx ng qnja"));
    assert_eq!(encrypt("true iS 42"), String::from("gehr vF 42"));
    assert_eq!(encrypt("こんばんは"), String::from("こんばんは"));
    assert_eq!(encrypt("Привет, world!"), String::from("Привет, jbeyq!"))
}

#[test]
fn test_rot13_decryption() {
    assert_eq!(encrypt("Nggnpx ng qnja"), String::from("Attack at dawn"));
    assert_eq!(encrypt("gehr vF 42"), String::from("true iS 42"));
    assert_eq!(encrypt("こんばんは"), String::from("こんばんは"));
    assert_eq!(encrypt("Привет, jbeyq!"), String::from("Привет, world!"))
}
