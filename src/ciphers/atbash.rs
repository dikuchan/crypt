pub fn encode(string: &str) -> String {
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
    assert_eq!(encode("Attack at dawn"), String::from("Zggzxp zg wzdm"));
    assert_eq!(encode("true iS 42"), String::from("gifv rH 42"));
    assert_eq!(encode("こんばんは"), String::from("こんばんは"));
    assert_eq!(encode("Привет, world!"), String::from("Привет, dliow!"))
}

#[test]
fn test_atbash_decryption() {
    assert_eq!(encode("Zggzxp zg wzdm"), String::from("Attack at dawn"));
    assert_eq!(encode("gifv rH 42"), String::from("true iS 42"));
    assert_eq!(encode("こんばんは"), String::from("こんばんは"));
    assert_eq!(encode("Привет, dliow!"), String::from("Привет, world!"))
}
