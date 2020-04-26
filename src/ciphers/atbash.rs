pub fn encrypt(string: &str) -> String {
    string.chars()
        .map(|c|
            match c {
                'a'..='z' => (0x7A - c as u8 + 0x61) as char, // 'z' - c + 'a'
                'A'..='Z' => (0x5A - c as u8 + 0x41) as char, // 'Z' - c + 'A'
                _ => c
            })
        .collect()
}

#[test]
fn test_atbash_encryption() {
    assert_eq!(encrypt("Attack at dawn"), String::from("Zggzxp zg wzdm"));
    assert_eq!(encrypt("true iS 42"), String::from("gifv rH 42"));
    assert_eq!(encrypt("こんばんは"), String::from("こんばんは"));
    assert_eq!(encrypt("Привет, world!"), String::from("Привет, dliow!"))
}

#[test]
fn test_atbash_decryption() {
    assert_eq!(encrypt("Zggzxp zg wzdm"), String::from("Attack at dawn"));
    assert_eq!(encrypt("gifv rH 42"), String::from("true iS 42"));
    assert_eq!(encrypt("こんばんは"), String::from("こんばんは"));
    assert_eq!(encrypt("Привет, dliow!"), String::from("Привет, world!"))
}
