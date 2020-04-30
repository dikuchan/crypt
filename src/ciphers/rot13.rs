/*
 *  [General]
 *
 *  The ROT13 cipher is a substitution cipher with a specific key where the letters of the alphabet are offset 13 places.
 *  I. e. all 'A's are replaced with 'N's, all 'B's are replaced with 'O's, and so on.
 *  It can also be thought of as a Caesar cipher with a shift of 13.
 *
 *  The ROT13 cipher offers almost no security, and can be broken very easily.
 *  Anyway, adversary can still break the message by determining the key using hill-climbing.
  * The ROT13 cipher is also an Caesar cipher with a key of 13, so breaking it as a Caesar cipher also works.
 *
 *  [Algorithm]
 *
 *  The ROT13 cipher is essentially a substitution cipher with a fixed key.
 *  If you know the cipher is ROT13, then no additional information is needed to decrypt the message.
 *  The substitution key is
 *
 *  ABCDEFGHIJKLMNOPQRSTUVWXYZ
 *  NOPQRSTUVWXYZABCDEFGHIJKLM
 *
 *  To encipher a message, replace the letter in the top row with the letter in the bottom row.
 *
 *  [Example]
 *
 *  In the example below, we encipher the message 'ATTACK AT DAWN'.
 *  The first letter we wish to encipher is 'A', which is above 'N', so the first ciphertext letter is 'N'.
 *  The next letter is 'T', which is above 'G', so that comes next.
 *  The whole message is enciphered
 *
 *  ATTACK AT DAWN
 *  NGGNPX NG QNJA
 *
 *  To decipher a message, the exact same procedure is followed.
 *  Find 'N' in the top row, which is 'A' in the bottom row.
 *  Continue until the whole message is deciphered.
 */

// Source: http://practicalcryptography.com

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
