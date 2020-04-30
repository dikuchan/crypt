/*
 *  [General]
 *
 *  The Atbash cipher is a substitution cipher with a specific key where the letters of the alphabet are reversed.
 *  I. e. all 'A's are replaced with 'Z's, all 'B's are replaced with 'Y's, and so on.
 *  It was originally used for the Hebrew alphabet, but can be used for any alphabet.
 *  The Atbash cipher offers almost no security, and can be broken very easily.
 *  Adversary can break the cipher by assuming it is a substitution cipher and determining the key using hill-climbing.
 *  The Atbash cipher is also an Affine cipher with a = 25 and b = 25, so breaking it as an affine cipher also works.
 *
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
 *  In the example, we encipher the message 'ATTACK AT DAWN'.
 *  The first letter we wish to encipher is 'A', which is above 'Z', so the first ciphertext letter is 'Z'.
 *  The next letter is 'T', which is above 'G', so that comes next.
 *  The whole message is enciphered:
 *
 *  ATTACK AT DAWN
 *  ZGGZXP ZG WZDM
 *
 *  To decipher a message, the exact same procedure is followed.
 *  Find 'Z' in the top row, which is 'A' in the bottom row.
 *  Continue until the whole message is deciphered.
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
