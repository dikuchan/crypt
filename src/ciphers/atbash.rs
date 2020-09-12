/*
 *  [Algorithm]
 *
 *  The Atbash cipher is essentially a substitution cipher with a fixed key.
 *  If you know the cipher is Atbash, then no additional information is needed to decrypt the message.
 *  The substitution key is
 *
 *  ABCDEFGHIJKLMNOPQRSTUVWXYZ
 *  ZYXWVUTSRQPONMLKJIHGFEDCBA
 *
 *  To encipher a message, replace the letter in the top row with the letter in the bottom row.
 *
 *  [Example]
 *
 *  The message is 'ATTACK AT DAWN'.
 *  The first letter we wish to encipher is 'A', which is above 'Z', so the first ciphertext letter is 'Z'.
 *  The next letter is 'T', which is above 'G', so that comes next.
 *  The whole message is enciphered:
 *
 *  ATTACK AT DAWN
 *  ZGGZXP ZG WZDM
 */

// Source: http://practicalcryptography.com

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
